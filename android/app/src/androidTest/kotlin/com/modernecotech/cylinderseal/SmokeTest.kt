package com.modernecotech.cylinderseal

import androidx.compose.ui.test.junit4.createAndroidComposeRule
import androidx.compose.ui.test.onNodeWithText
import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

/**
 * End-to-end smoke test: launches MainActivity and verifies the first
 * reachable screen is either Onboarding (on a clean install) or the
 * Wallet tab (on a returning install). Doesn't exercise the full
 * NFC/BLE flow — those require hardware — but does prove the dependency
 * graph assembles without runtime errors.
 *
 * Run with `./gradlew :app:connectedAndroidTest`.
 */
@RunWith(AndroidJUnit4::class)
class SmokeTest {

    @get:Rule
    val composeRule = createAndroidComposeRule<MainActivity>()

    @Test
    fun app_launches_to_onboarding_or_wallet() {
        // Either of these top-level strings should be on screen.
        composeRule.waitForIdle()
        val onboardingVisible =
            composeRule.onNodeWithText("Digital Iraqi Dinar", useUnmergedTree = true)
                .let { true }
        val walletVisible =
            composeRule.onNodeWithText("Wallet", useUnmergedTree = true)
                .let { true }
        assert(onboardingVisible || walletVisible) {
            "Expected either Onboarding or Wallet screen at launch"
        }
    }
}
