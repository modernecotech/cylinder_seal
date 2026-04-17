package com.modernecotech.cylinderseal.core.common

/**
 * Human-readable formatting for micro-OWC amounts.
 *
 * 1 OWC = 1_000_000 micro-OWC. Display shows up to 2 fractional digits and
 * is locale-agnostic to keep receipts reproducible.
 */
object MoneyFormat {
    private const val MICRO_PER_UNIT = 1_000_000L

    fun format(microOwc: Long, currency: String = "IQD"): String {
        val whole = microOwc / MICRO_PER_UNIT
        val frac = (microOwc % MICRO_PER_UNIT).let { r -> (if (r < 0) -r else r) }
        val fracDigits = (frac.toDouble() / MICRO_PER_UNIT * 100).toLong()
        return "%,d.%02d %s".format(whole, fracDigits, currency)
    }

    fun parseOwcString(s: String): Long? {
        val trimmed = s.replace(",", "").trim()
        val parts = trimmed.split(".")
        return when (parts.size) {
            1 -> parts[0].toLongOrNull()?.times(MICRO_PER_UNIT)
            2 -> {
                val whole = parts[0].toLongOrNull() ?: return null
                val fracStr = parts[1].padEnd(6, '0').take(6)
                val frac = fracStr.toLongOrNull() ?: return null
                whole * MICRO_PER_UNIT + frac
            }
            else -> null
        }
    }
}
