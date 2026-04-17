package com.modernecotech.cylinderseal.feature.onboarding

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.modernecotech.cylinderseal.core.common.toHex
import com.modernecotech.cylinderseal.core.cryptography.KeystoreManager
import com.modernecotech.cylinderseal.core.cryptography.WalletKeyManager
import com.modernecotech.cylinderseal.core.datastore.UserPreferences
import com.modernecotech.cylinderseal.core.ffi.MobileCore
import dagger.hilt.android.lifecycle.HiltViewModel
import javax.inject.Inject
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch

data class OnboardingState(
    val displayName: String = "",
    val phoneNumber: String = "",
    val pin: String = "",
    val pinConfirm: String = "",
    val step: Step = Step.Welcome,
    val publicKeyHex: String? = null,
    val errorMessage: String? = null,
    val busy: Boolean = false,
) {
    enum class Step { Welcome, Profile, SetPin, GenerateKeys, Done }
}

@HiltViewModel
class OnboardingViewModel @Inject constructor(
    private val keystore: KeystoreManager,
    private val wallet: WalletKeyManager,
    private val prefs: UserPreferences,
) : ViewModel() {

    private val _state = MutableStateFlow(OnboardingState())
    val state: StateFlow<OnboardingState> = _state

    fun setDisplayName(v: String) = update { it.copy(displayName = v) }
    fun setPhoneNumber(v: String) = update { it.copy(phoneNumber = v) }
    fun setPin(v: String) = update { it.copy(pin = v.filter(Char::isDigit).take(6)) }
    fun setPinConfirm(v: String) = update { it.copy(pinConfirm = v.filter(Char::isDigit).take(6)) }

    fun next() {
        val s = _state.value
        when (s.step) {
            OnboardingState.Step.Welcome -> update { it.copy(step = OnboardingState.Step.Profile) }
            OnboardingState.Step.Profile -> {
                if (s.displayName.isBlank()) {
                    update { it.copy(errorMessage = "Name required") }
                } else {
                    update { it.copy(errorMessage = null, step = OnboardingState.Step.SetPin) }
                }
            }
            OnboardingState.Step.SetPin -> {
                if (s.pin.length < 4) {
                    update { it.copy(errorMessage = "PIN must be 4-6 digits") }
                } else if (s.pin != s.pinConfirm) {
                    update { it.copy(errorMessage = "PINs don't match") }
                } else {
                    update { it.copy(errorMessage = null, step = OnboardingState.Step.GenerateKeys) }
                    generateKeys()
                }
            }
            OnboardingState.Step.GenerateKeys -> Unit
            OnboardingState.Step.Done -> Unit
        }
    }

    private fun generateKeys() {
        viewModelScope.launch {
            update { it.copy(busy = true) }
            try {
                keystore.ensureMasterKey()
                val publicKey = wallet.generateAndStore()
                val userId = MobileCore.userIdFromPublicKey(publicKey)
                prefs.completeOnboarding(
                    displayName = _state.value.displayName,
                    phoneNumber = _state.value.phoneNumber.takeIf { it.isNotBlank() },
                )
                update {
                    it.copy(
                        busy = false,
                        publicKeyHex = publicKey.toHex(),
                        step = OnboardingState.Step.Done,
                    )
                }
                timber.log.Timber.i("Onboarded user %s", userId)
            } catch (t: Throwable) {
                timber.log.Timber.e(t, "onboarding failed")
                update { it.copy(busy = false, errorMessage = "Key generation failed: ${t.message}") }
            }
        }
    }

    private inline fun update(transform: (OnboardingState) -> OnboardingState) {
        _state.value = transform(_state.value)
    }
}
