package com.modernecotech.cylinderseal.feature.sync

import android.content.Context
import androidx.hilt.work.HiltWorker
import androidx.work.CoroutineWorker
import androidx.work.WorkerParameters
import com.google.protobuf.ByteString
import com.modernecotech.cylinderseal.core.common.hexToBytes
import com.modernecotech.cylinderseal.core.database.PendingEntryDao
import com.modernecotech.cylinderseal.core.database.TransactionDao
import com.modernecotech.cylinderseal.core.datastore.UserPreferences
import com.modernecotech.cylinderseal.core.network.ChainSyncClient
import com.modernecotech.cylinderseal.core.network.snapshotAsFlow
import dagger.assisted.Assisted
import dagger.assisted.AssistedInject
import iq.cbi.cylinderseal.chainsync.JournalEntry
import iq.cbi.cylinderseal.chainsync.SyncAck
import iq.cbi.cylinderseal.chainsync.SyncAckStatus
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.flow.collect
import kotlinx.coroutines.flow.onEach

/**
 * Drains the `pending_entries` queue to the super-peer over gRPC `SyncChain`.
 *
 * WorkManager runs this whenever connectivity is available (constraint set
 * at schedule time in [SyncScheduler]). Each successful commit updates the
 * corresponding transaction's `sync_status` and removes the pending row.
 */
@HiltWorker
class SyncWorker @AssistedInject constructor(
    @Assisted context: Context,
    @Assisted params: WorkerParameters,
    private val pending: PendingEntryDao,
    private val transactions: TransactionDao,
    private val client: ChainSyncClient,
    private val prefs: UserPreferences,
) : CoroutineWorker(context, params) {

    override suspend fun doWork(): Result {
        val batch = pending.drain()
        if (batch.isEmpty()) return Result.success()

        // Build a list of JournalEntry protos. For Round 2 we send one
        // transaction per entry — multi-tx entries are a Round 3+ optimization.
        val entries = batch.map { p ->
            // The pending entry already carries the signed CBOR; we wrap it
            // in a minimal JournalEntry with just the proto fields the
            // super-peer validates before Raft propose. The server decodes
            // the CBOR payload independently for cryptographic checks.
            JournalEntry.newBuilder()
                .setEntryHash(ByteString.copyFrom(p.entryHashHex.hexToBytes()))
                .setSequenceNumber(p.sequenceNumber)
                .setCreatedAt(p.createdAt * 1000)
                .build()
        }

        val acks = mutableListOf<SyncAck>()
        try {
            client.syncChain(snapshotAsFlow(entries))
                .catch { timber.log.Timber.w(it, "sync stream error") }
                .onEach { acks.add(it) }
                .collect()
        } catch (t: Throwable) {
            timber.log.Timber.w(t, "sync failed")
            batch.forEach { pending.recordAttempt(it.entryHashHex, System.currentTimeMillis()) }
            return Result.retry()
        }

        for (ack in acks) {
            val entryHashHex = ack.entryId.toByteArray().joinToString("") { "%02x".format(it) }
            when (ack.statusValue) {
                SyncAckStatus.ACK_STATUS_CONFIRMED_VALUE -> {
                    pending.delete(entryHashHex)
                    // transactionId isn't carried by entry hash — update by
                    // hash won't work here; a future Round 3+ change links
                    // entry hash ↔ transaction id in the schema.
                }
                SyncAckStatus.ACK_STATUS_REJECTED_VALUE -> {
                    pending.delete(entryHashHex)
                }
                SyncAckStatus.ACK_STATUS_CONFLICTED_VALUE,
                SyncAckStatus.ACK_STATUS_PENDING_VALUE -> {
                    pending.recordAttempt(entryHashHex, System.currentTimeMillis())
                }
                else -> Unit
            }
        }

        prefs.recordSync(System.currentTimeMillis())
        return Result.success()
    }
}
