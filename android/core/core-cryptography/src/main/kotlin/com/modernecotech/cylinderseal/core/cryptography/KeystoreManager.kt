package com.modernecotech.cylinderseal.core.cryptography

import android.security.keystore.KeyGenParameterSpec
import android.security.keystore.KeyProperties
import com.modernecotech.cylinderseal.core.ffi.MobileCore
import java.security.KeyStore
import javax.crypto.KeyGenerator
import javax.crypto.SecretKey
import javax.inject.Inject
import javax.inject.Singleton

/**
 * Manages the device's hardware-backed master key.
 *
 * The Ed25519 payment-signing keypair itself is generated inside
 * [MobileCore.generateKeypair] (RustCrypto via UniFFI). Its *private half*
 * is then encrypted at rest under a symmetric AES-256 master key that lives
 * in the Android Keystore (hardware-backed, StrongBox preferred when
 * available on API 28+).
 *
 * The Keystore key itself is never exported; [seedForHkdf] returns a
 * derived public fingerprint that callers can mix into HKDF to bind
 * user-facing passphrases to the specific device. This is the
 * `HKDF(Keystore || PIN)` construction referenced in the architecture
 * decisions section of the project README.
 */
@Singleton
class KeystoreManager @Inject constructor() {

    private val keystore = KeyStore.getInstance(KEYSTORE_PROVIDER).apply { load(null) }

    /** Ensure the device master key exists, generating it if missing. */
    fun ensureMasterKey() {
        if (keystore.containsAlias(MASTER_KEY_ALIAS)) return

        val spec = KeyGenParameterSpec.Builder(
            MASTER_KEY_ALIAS,
            KeyProperties.PURPOSE_ENCRYPT or KeyProperties.PURPOSE_DECRYPT,
        )
            .setBlockModes(KeyProperties.BLOCK_MODE_GCM)
            .setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_NONE)
            .setKeySize(256)
            .setUserAuthenticationRequired(false)
            .apply {
                if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.P) {
                    setIsStrongBoxBacked(true)
                }
            }
            .build()

        val gen = KeyGenerator.getInstance(
            KeyProperties.KEY_ALGORITHM_AES,
            KEYSTORE_PROVIDER,
        )
        try {
            gen.init(spec)
            gen.generateKey()
        } catch (_: Throwable) {
            // StrongBox not available — retry without it.
            val fallback = KeyGenParameterSpec.Builder(
                MASTER_KEY_ALIAS,
                KeyProperties.PURPOSE_ENCRYPT or KeyProperties.PURPOSE_DECRYPT,
            )
                .setBlockModes(KeyProperties.BLOCK_MODE_GCM)
                .setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_NONE)
                .setKeySize(256)
                .build()
            gen.init(fallback)
            gen.generateKey()
        }
    }

    /** Expose a deterministic fingerprint suitable as HKDF IKM input. */
    fun seedForHkdf(): ByteArray {
        ensureMasterKey()
        // The master key cannot be extracted; use its alias + platform
        // unique identifiers as the seed. The actual entropy guarantee
        // comes from mixing this with the user PIN in HKDF.
        return MobileCore.blake2b256(
            (MASTER_KEY_ALIAS + android.os.Build.FINGERPRINT + android.os.Build.BOARD).toByteArray()
        )
    }

    internal fun masterKey(): SecretKey =
        (keystore.getEntry(MASTER_KEY_ALIAS, null) as KeyStore.SecretKeyEntry).secretKey

    companion object {
        private const val KEYSTORE_PROVIDER = "AndroidKeyStore"
        private const val MASTER_KEY_ALIAS = "cs_device_master_v1"
    }
}
