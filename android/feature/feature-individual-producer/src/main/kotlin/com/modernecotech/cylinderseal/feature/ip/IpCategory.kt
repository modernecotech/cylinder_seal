package com.modernecotech.cylinderseal.feature.ip

/**
 * The 8 CBI-approved Individual Producer categories. Mirrors
 * `cs-core::producer::IpCategory` and the `individual_producers.category`
 * CHECK in migration 20260420000001.
 */
enum class IpCategory(val wireName: String, val displayAr: String, val displayEn: String, val iconEmoji: String) {
    Food("food", "الغذاء", "Food", "\uD83C\uDF5E"),
    Crafts("crafts", "الحرف", "Crafts", "\uD83E\uDDF5"),
    Textiles("textiles", "النسيج", "Textiles", "\uD83E\uDDF6"),
    Repair("repair", "التصليح", "Repair", "\uD83D\uDD27"),
    Agriculture("agriculture", "الزراعة", "Agriculture", "\uD83C\uDF3E"),
    Services("services", "الخدمات", "Services", "\uD83D\uDEE0"),
    Construction("construction", "البناء", "Construction", "\uD83D\uDEA7"),
    Transport("transport", "النقل", "Transport", "\uD83D\uDE9B"),
}
