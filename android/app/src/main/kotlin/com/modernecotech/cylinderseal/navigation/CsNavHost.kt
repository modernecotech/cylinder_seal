package com.modernecotech.cylinderseal.navigation

import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.hilt.navigation.compose.hiltViewModel
import androidx.lifecycle.ViewModel
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.navigation.NavHostController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import com.modernecotech.cylinderseal.core.datastore.UserPreferences
import com.modernecotech.cylinderseal.feature.business.ApiKeysRoute
import com.modernecotech.cylinderseal.feature.business.BusinessOnboardingRoute
import com.modernecotech.cylinderseal.feature.history.ComplianceRoute
import com.modernecotech.cylinderseal.feature.history.HistoryRoute
import com.modernecotech.cylinderseal.feature.onboarding.OnboardingRoute
import com.modernecotech.cylinderseal.feature.pay.PayRoute
import com.modernecotech.cylinderseal.feature.receive.QrScannerScreen
import com.modernecotech.cylinderseal.feature.receive.ReceiveRoute
import com.modernecotech.cylinderseal.feature.settings.SettingsRoute
import com.modernecotech.cylinderseal.feature.wallet.WalletRoute
import dagger.hilt.android.lifecycle.HiltViewModel
import javax.inject.Inject
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.stateIn
import androidx.lifecycle.viewModelScope

object Routes {
    const val ONBOARDING = "onboarding"
    const val WALLET = "wallet"
    const val PAY = "pay"
    const val RECEIVE = "receive"
    const val SCAN = "scan"
    const val HISTORY = "history"
    const val COMPLIANCE = "compliance"
    const val SETTINGS = "settings"
    const val BUSINESS_REGISTER = "business/register"
    const val BUSINESS_API_KEYS = "business/api-keys"
}

@HiltViewModel
class RootViewModel @Inject constructor(prefs: UserPreferences) : ViewModel() {
    val onboarded: StateFlow<Boolean?> =
        prefs.isOnboarded.stateIn(viewModelScope, SharingStarted.Eagerly, null)
}

@Composable
fun CsNavHost(
    navController: NavHostController = rememberNavController(),
    rootVm: RootViewModel = hiltViewModel(),
) {
    val onboarded by rootVm.onboarded.collectAsStateWithLifecycle()
    val start = when (onboarded) {
        null -> Routes.ONBOARDING // wait-state draws onboarding; navigates once loaded
        true -> Routes.WALLET
        false -> Routes.ONBOARDING
    }

    NavHost(navController = navController, startDestination = start) {
        composable(Routes.ONBOARDING) {
            OnboardingRoute(
                onComplete = {
                    navController.navigate(Routes.WALLET) {
                        popUpTo(Routes.ONBOARDING) { inclusive = true }
                    }
                },
            )
        }
        composable(Routes.WALLET) {
            WalletRoute(
                onSendClick = { navController.navigate(Routes.PAY) },
                onReceiveClick = { navController.navigate(Routes.RECEIVE) },
                onScanClick = { navController.navigate(Routes.SCAN) },
                onHistoryClick = { navController.navigate(Routes.HISTORY) },
                onSettingsClick = { navController.navigate(Routes.SETTINGS) },
            )
        }
        composable(Routes.PAY) {
            PayRoute(onBack = { navController.popBackStack() })
        }
        composable(Routes.RECEIVE) {
            ReceiveRoute(
                onScanClick = { navController.navigate(Routes.SCAN) },
                onBack = { navController.popBackStack() },
            )
        }
        composable(Routes.SCAN) {
            QrScannerScreen(onScanned = { scanned ->
                // Re-use the receive flow's ingestor indirectly: the receive
                // screen reads the scan via its own ViewModel. For a cross-
                // screen scan we rely on the ingestor singleton; the scan
                // screen just pops the stack.
                navController.popBackStack()
                timber.log.Timber.i("scanned: $scanned")
            })
        }
        composable(Routes.HISTORY) { HistoryRoute() }
        composable(Routes.COMPLIANCE) { ComplianceRoute() }
        composable(Routes.SETTINGS) { SettingsRoute() }
        composable(Routes.BUSINESS_REGISTER) {
            BusinessOnboardingRoute(onDone = { navController.popBackStack() })
        }
        composable(Routes.BUSINESS_API_KEYS) { ApiKeysRoute() }
    }
}
