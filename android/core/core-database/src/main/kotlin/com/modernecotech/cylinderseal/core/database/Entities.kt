package com.modernecotech.cylinderseal.core.database

import androidx.room.ColumnInfo
import androidx.room.Entity
import androidx.room.Index
import androidx.room.PrimaryKey

@Entity(tableName = "transactions", indices = [Index(value = ["timestamp_utc"]), Index(value = ["sync_status"])])
data class TransactionEntity(
    @PrimaryKey val transactionId: String,
    @ColumnInfo(name = "amount_micro_owc") val amountMicroOwc: Long,
    @ColumnInfo(name = "direction") val direction: String, // INCOMING/OUTGOING
    @ColumnInfo(name = "counterparty_pk_hex") val counterpartyPublicKeyHex: String,
    @ColumnInfo(name = "counterparty_name") val counterpartyName: String?,
    @ColumnInfo(name = "currency") val currency: String,
    @ColumnInfo(name = "channel") val channel: String,       // NFC/BLE/ONLINE
    @ColumnInfo(name = "memo") val memo: String,
    @ColumnInfo(name = "timestamp_utc") val timestampUtc: Long,
    @ColumnInfo(name = "sync_status") val syncStatus: String, // PENDING/CONFIRMED/CONFLICTED/REJECTED
    @ColumnInfo(name = "latitude") val latitude: Double,
    @ColumnInfo(name = "longitude") val longitude: Double,
    @ColumnInfo(name = "cbor") val cborPayload: ByteArray,
) {
    override fun equals(other: Any?): Boolean =
        other is TransactionEntity && transactionId == other.transactionId
    override fun hashCode(): Int = transactionId.hashCode()
}

@Entity(tableName = "pending_entries", indices = [Index(value = ["sequence_number"], unique = true)])
data class PendingEntryEntity(
    @PrimaryKey val entryHashHex: String,
    @ColumnInfo(name = "sequence_number") val sequenceNumber: Long,
    @ColumnInfo(name = "cbor") val cborPayload: ByteArray,
    @ColumnInfo(name = "created_at") val createdAt: Long,
    @ColumnInfo(name = "last_attempt_at") val lastAttemptAt: Long?,
    @ColumnInfo(name = "attempt_count") val attemptCount: Int,
) {
    override fun equals(other: Any?): Boolean =
        other is PendingEntryEntity && entryHashHex == other.entryHashHex
    override fun hashCode(): Int = entryHashHex.hashCode()
}

@Entity(tableName = "contacts")
data class ContactEntity(
    @PrimaryKey val publicKeyHex: String,
    @ColumnInfo(name = "display_name") val displayName: String,
    @ColumnInfo(name = "phone_number") val phoneNumber: String?,
    @ColumnInfo(name = "last_seen_at") val lastSeenAt: Long?,
)

@Entity(tableName = "nonce_chain")
data class NonceChainEntity(
    @PrimaryKey @ColumnInfo(name = "counter") val counter: Long,
    @ColumnInfo(name = "nonce_hex") val nonceHex: String,
    @ColumnInfo(name = "created_at") val createdAt: Long,
)
