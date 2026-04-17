package com.modernecotech.cylinderseal.core.cryptography

import android.content.Context
import com.google.crypto.tink.aead.AesGcmKeyManager
import com.google.crypto.tink.subtle.AesGcmJce
import com.modernecotech.cylinderseal.core.ffi.MobileCore
import dagger.hilt.android.qualifiers.ApplicationContext
import java.io.File
import javax.inject.Inject
import javax.inject.Singleton

/**
 * On-device management of the Ed25519 payment keypair.
 *
 * Flow:
 * 1. On first run, [KeystoreManager.ensureMasterKey] creates the
 *    hardware-backed AES-256 GCM master key.
 * 2. [generateAndStore] produces a fresh Ed25519 keypair via the Rust
 *    `cs-mobile-core` and encrypts the private half under the Keystore
 *    master key, writing it to an app-private file.
 * 3. [loadPrivateKey] decrypts it when the user authorizes a payment.
 */
@Singleton
class WalletKeyManager @Inject constructor(
    @ApplicationContext private val ctx: Context,
    private val keystore: KeystoreManager,
) {
    private val walletFile: File get() = File(ctx.filesDir, WALLET_FILE)
    private val publicKeyFile: File get() = File(ctx.filesDir, PUBLIC_KEY_FILE)

    fun hasWallet(): Boolean = walletFile.exists() && publicKeyFile.exists()

    /** Generate a new keypair and persist it. Returns the public key. */
    fun generateAndStore(): ByteArray {
        val kp = MobileCore.generateKeypair()
        val aead = AesGcmJce(deriveAesKey())
        val ciphertext = aead.encrypt(kp.privateKey, AAD)
        walletFile.writeBytes(ciphertext)
        publicKeyFile.writeBytes(kp.publicKey)
        return kp.publicKey
    }

    fun loadPublicKey(): ByteArray = publicKeyFile.readBytes()

    /**
     * Decrypt and return the private key. Callers MUST zero out the
     * returned array as soon as the signing operation completes.
     */
    fun loadPrivateKey(): ByteArray {
        val aead = AesGcmJce(deriveAesKey())
        val ciphertext = walletFile.readBytes()
        return aead.decrypt(ciphertext, AAD)
    }

    private fun deriveAesKey(): ByteArray {
        // Derive the wrap-key from the Keystore-bound seed. The SQLCipher
        // DB key uses a different HKDF `info` string so the two are
        // distinct even though they share the same IKM.
        return Hkdf.derive(
            ikm = keystore.seedForHkdf(),
            salt = AAD,
            info = "cs:wallet-key-wrap:v1".toByteArray(),
            length = 32,
        )
    }

    companion object {
        private const val WALLET_FILE = "wallet.bin"
        private const val PUBLIC_KEY_FILE = "public.bin"
        private val AAD = "cs.wallet.v1".toByteArray()
    }
}
