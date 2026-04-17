package com.modernecotech.cylinderseal.core.cryptography

import javax.crypto.Mac
import javax.crypto.spec.SecretKeySpec

/** HKDF-SHA256 as specified in RFC 5869. */
object Hkdf {
    private const val ALGO = "HmacSHA256"
    private const val HASH_LEN = 32

    /**
     * Derives a key of length [length] from:
     *  - `ikm` (initial key material; typically Keystore-bound seed),
     *  - `salt` (non-secret; we use user PIN here),
     *  - `info` (context string like "cs:sqlcipher:v1").
     */
    fun derive(ikm: ByteArray, salt: ByteArray, info: ByteArray, length: Int): ByteArray {
        require(length in 1..(255 * HASH_LEN)) { "invalid length" }
        val prk = extract(salt, ikm)
        return expand(prk, info, length)
    }

    private fun extract(salt: ByteArray, ikm: ByteArray): ByteArray {
        val mac = Mac.getInstance(ALGO)
        val actualSalt = if (salt.isEmpty()) ByteArray(HASH_LEN) else salt
        mac.init(SecretKeySpec(actualSalt, ALGO))
        return mac.doFinal(ikm)
    }

    private fun expand(prk: ByteArray, info: ByteArray, length: Int): ByteArray {
        val mac = Mac.getInstance(ALGO)
        mac.init(SecretKeySpec(prk, ALGO))
        val n = (length + HASH_LEN - 1) / HASH_LEN
        var last = ByteArray(0)
        val out = ByteArray(length)
        var filled = 0
        for (i in 1..n) {
            mac.reset()
            mac.update(last)
            mac.update(info)
            mac.update(byteArrayOf(i.toByte()))
            last = mac.doFinal()
            val take = minOf(HASH_LEN, length - filled)
            System.arraycopy(last, 0, out, filled, take)
            filled += take
        }
        return out
    }
}
