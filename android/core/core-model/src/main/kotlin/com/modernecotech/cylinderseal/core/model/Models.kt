package com.modernecotech.cylinderseal.core.model

import kotlinx.serialization.Serializable

/** Micro-OWC: internal integer unit. 1 OWC = 1_000_000 micro-OWC. */
typealias MicroOwc = Long

@Serializable
enum class KycTier { ANONYMOUS, PHONE_VERIFIED, FULL_KYC }

@Serializable
enum class PaymentChannel { NFC, BLE, ONLINE }

@Serializable
enum class LocationSource { UNSPECIFIED, GPS, NETWORK, LAST_KNOWN, OFFLINE }

@Serializable
enum class SyncStatus { PENDING, CONFIRMED, CONFLICTED, REJECTED }

/** User profile as known on the device. */
@Serializable
data class Profile(
    val userId: String,          // UUID string
    val displayName: String,
    val phoneNumber: String?,
    val publicKeyHex: String,
    val kycTier: KycTier,
    val balanceOwc: MicroOwc,
    val createdAt: Long,         // epoch millis
)

/** A transaction as displayed in the local history screen. */
@Serializable
data class TransactionRecord(
    val transactionId: String,
    val amountMicroOwc: MicroOwc,
    val direction: Direction,
    val counterpartyPublicKeyHex: String,
    val counterpartyName: String?,
    val currency: String,
    val channel: PaymentChannel,
    val memo: String,
    val timestampUtc: Long,
    val syncStatus: SyncStatus,
    val latitude: Double,
    val longitude: Double,
)

@Serializable
enum class Direction { INCOMING, OUTGOING }

/** An entry awaiting sync to the super-peer. */
@Serializable
data class PendingEntry(
    val entryHashHex: String,
    val sequenceNumber: Long,
    val cborPayload: ByteArray,
    val createdAt: Long,
    val lastAttemptAt: Long?,
    val attemptCount: Int,
) {
    override fun equals(other: Any?): Boolean =
        other is PendingEntry &&
            entryHashHex == other.entryHashHex &&
            sequenceNumber == other.sequenceNumber

    override fun hashCode(): Int = entryHashHex.hashCode() xor sequenceNumber.hashCode()
}

/** A registered merchant visible to the payment flow. */
@Serializable
data class MerchantInfo(
    val merchantId: String,
    val displayName: String,
    val iraqiContentPct: Int,
    val category: String,
    val tier: MerchantTier,
    val feePercent: Double,
)

@Serializable
enum class MerchantTier { TIER1, TIER2, TIER3, TIER4, UNCLASSIFIED }
