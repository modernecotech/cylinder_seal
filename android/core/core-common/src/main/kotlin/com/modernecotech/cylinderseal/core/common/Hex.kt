package com.modernecotech.cylinderseal.core.common

/** Encode bytes to lower-case hex. */
fun ByteArray.toHex(): String = joinToString("") { "%02x".format(it) }

/** Decode hex string (optional "0x" prefix) back into bytes. Throws if invalid. */
fun String.hexToBytes(): ByteArray {
    val clean = removePrefix("0x")
    require(clean.length % 2 == 0) { "hex string must have even length" }
    return ByteArray(clean.length / 2) {
        val hi = Character.digit(clean[it * 2], 16)
        val lo = Character.digit(clean[it * 2 + 1], 16)
        require(hi >= 0 && lo >= 0) { "invalid hex char" }
        ((hi shl 4) or lo).toByte()
    }
}
