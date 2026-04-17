package com.modernecotech.cylinderseal.feature.business

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.modernecotech.cylinderseal.core.common.toHex
import com.modernecotech.cylinderseal.core.cryptography.WalletKeyManager
import com.modernecotech.cylinderseal.core.ffi.MobileCore
import dagger.hilt.android.lifecycle.HiltViewModel
import javax.inject.Inject
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch

enum class BusinessKind { POS, ELECTRONIC }

data class BusinessOnboardingState(
    val kind: BusinessKind = BusinessKind.POS,
    val legalName: String = "",
    val commercialRegistrationId: String = "",
    val taxId: String = "",
    val industryCode: String = "",
    val registeredAddress: String = "",
    val contactEmail: String = "",
    val submitted: Boolean = false,
    val statusMessage: String? = null,
    val errorMessage: String? = null,
    val busy: Boolean = false,
)

@HiltViewModel
class BusinessOnboardingViewModel @Inject constructor(
    private val wallet: WalletKeyManager,
    private val api: BusinessApi,
) : ViewModel() {
    private val _state = MutableStateFlow(BusinessOnboardingState())
    val state: StateFlow<BusinessOnboardingState> = _state

    fun setKind(v: BusinessKind) = update { it.copy(kind = v) }
    fun setLegalName(v: String) = update { it.copy(legalName = v) }
    fun setRegistration(v: String) = update { it.copy(commercialRegistrationId = v) }
    fun setTaxId(v: String) = update { it.copy(taxId = v) }
    fun setIndustry(v: String) = update { it.copy(industryCode = v.filter(Char::isDigit).take(4)) }
    fun setAddress(v: String) = update { it.copy(registeredAddress = v) }
    fun setEmail(v: String) = update { it.copy(contactEmail = v.trim()) }

    fun submit() {
        val s = _state.value
        if (s.legalName.isBlank() || s.commercialRegistrationId.isBlank() ||
            s.taxId.isBlank() || s.industryCode.isBlank() ||
            s.registeredAddress.isBlank() || s.contactEmail.isBlank()
        ) {
            update { it.copy(errorMessage = "All fields are required") }
            return
        }

        viewModelScope.launch {
            update { it.copy(busy = true, errorMessage = null) }
            try {
                val publicKey = wallet.loadPublicKey()
                val userId = MobileCore.userIdFromPublicKey(publicKey)
                val accountType = when (s.kind) {
                    BusinessKind.POS -> "business_pos"
                    BusinessKind.ELECTRONIC -> "business_electronic"
                }
                val resp = api.register(
                    RegisterRequest(
                        user_id = userId,
                        account_type = accountType,
                        legal_name = s.legalName,
                        commercial_registration_id = s.commercialRegistrationId,
                        tax_id = s.taxId,
                        industry_code = s.industryCode,
                        registered_address = s.registeredAddress,
                        contact_email = s.contactEmail,
                        authorized_signer_public_keys_hex = listOf(publicKey.toHex()),
                    )
                )
                update {
                    it.copy(
                        busy = false,
                        submitted = true,
                        statusMessage = "Registered: ${resp.status}. CBI ops will verify your commercial registration before activation.",
                    )
                }
            } catch (t: Throwable) {
                update {
                    it.copy(busy = false, errorMessage = t.message ?: "Registration failed")
                }
            }
        }
    }

    private inline fun update(transform: (BusinessOnboardingState) -> BusinessOnboardingState) {
        _state.value = transform(_state.value)
    }
}
