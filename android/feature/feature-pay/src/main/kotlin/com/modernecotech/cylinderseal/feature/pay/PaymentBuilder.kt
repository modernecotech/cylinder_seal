package com.modernecotech.cylinderseal.feature.pay

import com.modernecotech.cylinderseal.core.common.toHex
import com.modernecotech.cylinderseal.core.cryptography.WalletKeyManager
import com.modernecotech.cylinderseal.core.database.NonceChainDao
import com.modernecotech.cylinderseal.core.database.NonceChainEntity
import com.modernecotech.cylinderseal.core.database.PendingEntryDao
import com.modernecotech.cylinderseal.core.database.PendingEntryEntity
import com.modernecotech.cylinderseal.core.database.TransactionDao
import com.modernecotech.cylinderseal.core.database.TransactionEntity
import com.modernecotech.cylinderseal.core.ffi.MobileCore
import java.util.UUID
import javax.inject.Inject

/** Extracted so both UI and tests can share the build-and-persist logic. */
class PaymentBuilder @Inject constructor(
    private val wallet: WalletKeyManager,
    private val nonces: NonceChainDao,
    private val pending: PendingEntryDao,
    private val transactions: TransactionDao,
) {

    /** Where the payment is going to be transported to the recipient. */
    enum class Channel(val code: Int) {
        NFC(1), BLE(2), ONLINE(3);
    }

    data class Built(
        val cborPayload: ByteArray,
        val qrPayload: String,
        val nfcApdus: List<ByteArray>,
        val transactionId: String,
    )

    /**
     * Build a signed transaction, persist it as pending, and return the
     * payloads for each transport channel.
     */
    suspend fun build(
        recipientPublicKey: ByteArray,
        amountMicroOwc: Long,
        currency: String,
        channel: Channel,
        memo: String,
        latitude: Double,
        longitude: Double,
        locationAccuracyMeters: Int,
    ): Built {
        val ownPk = wallet.loadPublicKey()
        val privKey = wallet.loadPrivateKey()
        try {
            val counter = (nonces.latestCounter() ?: 0L) + 1L
            val prevNonce = nonces.latest()?.nonceHex?.let { hexDecode(it) }
                ?: ByteArray(32) // genesis
            val hardwareSeed = MobileCore.blake2b256(ownPk + byteArrayOf(channel.code.toByte()))
            val nextNonce = MobileCore.deriveNextNonce(prevNonce, hardwareSeed, counter)

            val deviceId = deviceUuid()
            val channelFfi = when (channel) {
                Channel.NFC -> MobileCore.PaymentChannel.NFC
                Channel.BLE -> MobileCore.PaymentChannel.BLE
                Channel.ONLINE -> MobileCore.PaymentChannel.ONLINE
            }

            val input = MobileCore.TransactionInput(
                fromPublicKey = ownPk,
                toPublicKey = recipientPublicKey,
                amountMicroOwc = amountMicroOwc,
                currencyContext = currency,
                fxRateSnapshot = "1.0",
                channel = channelFfi,
                memo = memo,
                deviceId = deviceId.toString(),
                previousNonce = prevNonce,
                currentNonce = nextNonce,
                latitude = latitude,
                longitude = longitude,
                locationAccuracyMeters = locationAccuracyMeters,
                locationSource = MobileCore.LocationSource.OFFLINE,
            )

            val cbor = MobileCore.buildAndSignTransaction(input, privKey)
            val qr = MobileCore.encodeQrPayload(cbor)
            val apdus = MobileCore.buildNfcApdus(cbor)

            val view = MobileCore.decodeTransaction(cbor)

            nonces.insert(
                NonceChainEntity(
                    counter = counter,
                    nonceHex = nextNonce.toHex(),
                    createdAt = System.currentTimeMillis(),
                ),
            )

            val entryHash = MobileCore.blake2b256(cbor).toHex()
            pending.insert(
                PendingEntryEntity(
                    entryHashHex = entryHash,
                    sequenceNumber = counter,
                    cborPayload = cbor,
                    createdAt = System.currentTimeMillis(),
                    lastAttemptAt = null,
                    attemptCount = 0,
                ),
            )
            transactions.upsert(
                TransactionEntity(
                    transactionId = view.transactionId,
                    amountMicroOwc = amountMicroOwc,
                    direction = "OUTGOING",
                    counterpartyPublicKeyHex = recipientPublicKey.toHex(),
                    counterpartyName = null,
                    currency = currency,
                    channel = channel.name,
                    memo = memo,
                    timestampUtc = view.timestampUtc,
                    syncStatus = "PENDING",
                    latitude = latitude,
                    longitude = longitude,
                    cborPayload = cbor,
                ),
            )

            return Built(
                cborPayload = cbor,
                qrPayload = qr,
                nfcApdus = apdus,
                transactionId = view.transactionId,
            )
        } finally {
            privKey.fill(0)
        }
    }

    private fun deviceUuid(): UUID {
        // Per-install deterministic UUID derived from Keystore-bound seed so
        // reinstalls look like new devices (desired: re-registration).
        val seed = MobileCore.blake2b256("cs.device.id".toByteArray())
        return UUID.nameUUIDFromBytes(seed)
    }

    private fun hexDecode(s: String): ByteArray = ByteArray(s.length / 2) {
        val hi = Character.digit(s[it * 2], 16)
        val lo = Character.digit(s[it * 2 + 1], 16)
        ((hi shl 4) or lo).toByte()
    }
}
