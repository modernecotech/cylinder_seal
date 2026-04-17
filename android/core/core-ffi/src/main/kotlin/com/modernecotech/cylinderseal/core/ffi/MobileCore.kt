package com.modernecotech.cylinderseal.core.ffi

/**
 * Thin Kotlin facade over the UniFFI-generated bindings for `cs-mobile-core`.
 *
 * The UniFFI bindings live in the package `uniffi.cs_mobile_core` and are
 * generated from `crates/cs-mobile-core/src/cs_mobile_core.udl` by running:
 *
 *     cargo run --bin uniffi-bindgen -- generate \
 *         crates/cs-mobile-core/src/cs_mobile_core.udl \
 *         --language kotlin \
 *         --out-dir android/core/core-ffi/src/main/kotlin
 *
 * and copying the compiled `libcs_mobile_core.so` for each ABI into
 * `android/core/core-ffi/src/main/jniLibs/<abi>/`.
 *
 * This wrapper keeps callers on Kotlin-idiomatic types (Kotlin `Result`,
 * Kotlin data classes) and hides JNA/UniFFI details from feature modules.
 */
object MobileCore {

    data class Keypair(val publicKey: ByteArray, val privateKey: ByteArray) {
        override fun equals(other: Any?): Boolean =
            other is Keypair &&
                publicKey.contentEquals(other.publicKey) &&
                privateKey.contentEquals(other.privateKey)
        override fun hashCode(): Int = publicKey.contentHashCode() xor privateKey.contentHashCode()
    }

    data class TransactionInput(
        val fromPublicKey: ByteArray,
        val toPublicKey: ByteArray,
        val amountMicroOwc: Long,
        val currencyContext: String,
        val fxRateSnapshot: String,
        val channel: PaymentChannel,
        val memo: String,
        val deviceId: String,
        val previousNonce: ByteArray,
        val currentNonce: ByteArray,
        val latitude: Double,
        val longitude: Double,
        val locationAccuracyMeters: Int,
        val locationSource: LocationSource,
    )

    data class TransactionView(
        val transactionId: String,
        val fromPublicKey: ByteArray,
        val toPublicKey: ByteArray,
        val amountMicroOwc: Long,
        val currencyContext: String,
        val timestampUtc: Long,
        val memo: String,
        val channel: PaymentChannel,
        val deviceId: String,
        val signatureValid: Boolean,
    )

    enum class PaymentChannel(val code: Int) {
        NFC(1),
        BLE(2),
        ONLINE(3);

        companion object {
            fun fromCode(code: Int): PaymentChannel = values().first { it.code == code }
        }
    }

    enum class LocationSource(val code: Int) {
        UNSPECIFIED(0),
        GPS(1),
        NETWORK(2),
        LAST_KNOWN(3),
        OFFLINE(4);

        companion object {
            fun fromCode(code: Int): LocationSource = values().first { it.code == code }
        }
    }

    // --- Facade methods --------------------------------------------------
    //
    // Each method delegates to the generated `uniffi.cs_mobile_core` functions.
    // Generation places those functions under `uniffi.cs_mobile_core` with
    // the same names as the UDL. The calls are wrapped so only generated
    // artifacts need to change if the UDL evolves.

    fun generateKeypair(): Keypair {
        val kp = uniffi.cs_mobile_core.generateKeypair()
        return Keypair(kp.publicKey, kp.privateKey)
    }

    fun blake2b256(data: ByteArray): ByteArray =
        uniffi.cs_mobile_core.blake2b256(data)

    fun userIdFromPublicKey(publicKey: ByteArray): String =
        uniffi.cs_mobile_core.userIdFromPublicKey(publicKey)

    fun buildAndSignTransaction(input: TransactionInput, privateKey: ByteArray): ByteArray {
        val ffiInput = uniffi.cs_mobile_core.TransactionInput(
            fromPublicKey = input.fromPublicKey,
            toPublicKey = input.toPublicKey,
            amountMicroOwc = input.amountMicroOwc,
            currencyContext = input.currencyContext,
            fxRateSnapshot = input.fxRateSnapshot,
            channel = input.channel.code,
            memo = input.memo,
            deviceId = input.deviceId,
            previousNonce = input.previousNonce,
            currentNonce = input.currentNonce,
            latitude = input.latitude,
            longitude = input.longitude,
            locationAccuracyMeters = input.locationAccuracyMeters,
            locationSource = input.locationSource.code,
        )
        return uniffi.cs_mobile_core.buildAndSignTransaction(ffiInput, privateKey)
    }

    fun decodeTransaction(cbor: ByteArray): TransactionView {
        val view = uniffi.cs_mobile_core.decodeTransaction(cbor)
        return TransactionView(
            transactionId = view.transactionId,
            fromPublicKey = view.fromPublicKey,
            toPublicKey = view.toPublicKey,
            amountMicroOwc = view.amountMicroOwc,
            currencyContext = view.currencyContext,
            timestampUtc = view.timestampUtc,
            memo = view.memo,
            channel = PaymentChannel.fromCode(view.channel),
            deviceId = view.deviceId,
            signatureValid = view.signatureValid,
        )
    }

    fun deriveNextNonce(prevNonce: ByteArray, hardwareSeed: ByteArray, counter: Long): ByteArray =
        uniffi.cs_mobile_core.deriveNextNonce(prevNonce, hardwareSeed, counter.toULong())

    fun encodeQrPayload(cbor: ByteArray): String =
        uniffi.cs_mobile_core.encodeQrPayload(cbor)

    fun decodeQrPayload(qr: String): ByteArray =
        uniffi.cs_mobile_core.decodeQrPayload(qr)

    fun buildNfcApdus(cbor: ByteArray): List<ByteArray> =
        uniffi.cs_mobile_core.buildNfcApdus(cbor)
}
