package com.modernecotech.cylinderseal.core.common

import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class MoneyTest {

    @Test
    fun `1 OWC formats as 1 00`() {
        val formatted = MoneyFormat.format(1_000_000L, "IQD")
        assertEquals("1.00 IQD", formatted)
    }

    @Test
    fun `fractional amounts round to 2 digits`() {
        // 1_500_000 micro = 1.50 OWC
        assertEquals("1.50 IQD", MoneyFormat.format(1_500_000L, "IQD"))
    }

    @Test
    fun `thousand separator applied to whole part`() {
        // 1_000_000_000 micro = 1000 OWC
        val s = MoneyFormat.format(1_000_000_000L, "IQD")
        assertEquals("1,000.00 IQD", s)
    }

    @Test
    fun `parse roundtrips integer input`() {
        assertEquals(5_000_000L, MoneyFormat.parseOwcString("5"))
    }

    @Test
    fun `parse accepts fractional input`() {
        assertEquals(1_500_000L, MoneyFormat.parseOwcString("1.5"))
    }

    @Test
    fun `parse strips comma grouping`() {
        assertEquals(1_000_000_000L, MoneyFormat.parseOwcString("1,000"))
    }

    @Test
    fun `parse rejects garbage`() {
        assertNull(MoneyFormat.parseOwcString("abc"))
    }
}
