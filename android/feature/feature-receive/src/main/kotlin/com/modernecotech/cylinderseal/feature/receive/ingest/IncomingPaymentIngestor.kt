package com.modernecotech.cylinderseal.feature.receive.ingest

import com.modernecotech.cylinderseal.core.common.toHex
import com.modernecotech.cylinderseal.core.database.PendingEntryDao
import com.modernecotech.cylinderseal.core.database.PendingEntryEntity
import com.modernecotech.cylinderseal.core.database.TransactionDao
import com.modernecotech.cylinderseal.core.database.TransactionEntity
import com.modernecotech.cylinderseal.core.ffi.MobileCore
import javax.inject.Inject
import javax.inject.Singleton
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.launch

/**
 * Handles a freshly-received CBOR transaction payload (from NFC, BLE, or
 * QR scan) by:
 *   1. Decoding and verifying the signature via `MobileCore.decodeTransaction`.
 *   2. Storing the raw CBOR in `pending_entries` for later super-peer sync.
 *   3. Recording a `transactions` row so the UI shows the incoming payment
 *      immediately.
 *
 * Returning `false` from `onPayload` signals the caller to reject (for NFC
 * this translates to an error status word so the sender knows to retry).
 */
@Singleton
class IncomingPaymentIngestor @Inject constructor(
    private val pending: PendingEntryDao,
    private val transactions: TransactionDao,
) {
    private val scope = CoroutineScope(SupervisorJob() + Dispatchers.IO)

    /** Returns true if the payload was accepted. Safe to call from any thread. */
    fun onPayload(cbor: ByteArray): Boolean {
        val view = try {
            MobileCore.decodeTransaction(cbor)
        } catch (_: Throwable) {
            timber.log.Timber.w("incoming payload decode failed")
            return false
        }
        if (!view.signatureValid) {
            timber.log.Timber.w("incoming payload signature invalid")
            return false
        }
        val entryHash = MobileCore.blake2b256(cbor).toHex()
        scope.launch {
            val nextSeq = (pending.latestSequence() ?: 0L) + 1L
            pending.insert(
                PendingEntryEntity(
                    entryHashHex = entryHash,
                    sequenceNumber = nextSeq,
                    cborPayload = cbor,
                    createdAt = System.currentTimeMillis(),
                    lastAttemptAt = null,
                    attemptCount = 0,
                ),
            )
            transactions.upsert(
                TransactionEntity(
                    transactionId = view.transactionId,
                    amountMicroOwc = view.amountMicroOwc,
                    direction = "INCOMING",
                    counterpartyPublicKeyHex = view.fromPublicKey.toHex(),
                    counterpartyName = null,
                    currency = view.currencyContext,
                    channel = view.channel.name,
                    memo = view.memo,
                    timestampUtc = view.timestampUtc,
                    syncStatus = "PENDING",
                    latitude = 0.0,
                    longitude = 0.0,
                    cborPayload = cbor,
                ),
            )
        }
        return true
    }
}
