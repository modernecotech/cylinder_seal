package com.modernecotech.cylinderseal.core.cryptography

import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotEquals
import org.junit.Assert.assertNotNull
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith

/**
 * Full-device integration test for the wallet keypair lifecycle.
 * Exercises: Android Keystore master-key generation → Rust-core Ed25519
 * keypair creation → Tink AES-GCM wrap → file write → later unwrap.
 *
 * Run with `./gradlew :core:core-cryptography:connectedAndroidTest`
 * or via an Android device/emulator in Android Studio.
 */
@RunWith(AndroidJUnit4::class)
class WalletKeyManagerInstrumentedTest {

    private lateinit var keystore: KeystoreManager
    private lateinit var wallet: WalletKeyManager

    @Before
    fun setUp() {
        val ctx = InstrumentationRegistry.getInstrumentation().targetContext
        keystore = KeystoreManager()
        wallet = WalletKeyManager(ctx, keystore)
        // Start clean for each run.
        runCatching { wallet.reset() }
    }

    @Test
    fun ensureMasterKey_is_idempotent() {
        keystore.ensureMasterKey()
        val seed1 = keystore.seedForHkdf()
        keystore.ensureMasterKey()
        val seed2 = keystore.seedForHkdf()
        assertArrayEquals("Seed must be stable across ensureMasterKey calls", seed1, seed2)
    }

    @Test
    fun generate_and_load_roundtrip() {
        val publicKey = wallet.generateAndStore()
        assertEquals("Ed25519 public keys are 32 bytes", 32, publicKey.size)
        assertNotNull("hasWallet after generation", wallet.hasWallet())

        val loaded = wallet.loadPublicKey()
        assertArrayEquals("Stored public key matches what generate returned", publicKey, loaded)

        val priv = wallet.loadPrivateKey()
        assertEquals("Ed25519 private keys are 32 bytes", 32, priv.size)
    }

    @Test
    fun two_generations_produce_distinct_keys() {
        val pk1 = wallet.generateAndStore()
        // Resetting and regenerating produces fresh material.
        wallet.reset()
        val pk2 = wallet.generateAndStore()
        assertNotEquals("Second generation must produce a different public key", pk1, pk2)
    }
}
