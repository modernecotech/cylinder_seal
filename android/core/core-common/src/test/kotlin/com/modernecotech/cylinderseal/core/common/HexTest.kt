package com.modernecotech.cylinderseal.core.common

import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertEquals
import org.junit.Assert.assertThrows
import org.junit.Test

class HexTest {

    @Test
    fun `empty bytes encodes to empty string`() {
        assertEquals("", byteArrayOf().toHex())
    }

    @Test
    fun `single byte encodes as lowercase hex`() {
        assertEquals("0f", byteArrayOf(0x0f).toHex())
        assertEquals("ff", byteArrayOf(0xff.toByte()).toHex())
    }

    @Test
    fun `roundtrip preserves bytes`() {
        val bytes = byteArrayOf(0x00, 0x7f, 0xff.toByte(), 0x10, 0x20)
        val hex = bytes.toHex()
        assertArrayEquals(bytes, hex.hexToBytes())
    }

    @Test
    fun `hex decoder accepts 0x prefix`() {
        assertArrayEquals(byteArrayOf(0xaa.toByte(), 0xbb.toByte()), "0xaabb".hexToBytes())
    }

    @Test
    fun `hex decoder rejects odd length`() {
        assertThrows(IllegalArgumentException::class.java) { "abc".hexToBytes() }
    }

    @Test
    fun `hex decoder rejects non-hex chars`() {
        assertThrows(IllegalArgumentException::class.java) { "zzzz".hexToBytes() }
    }
}
