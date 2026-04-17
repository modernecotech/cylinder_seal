package com.modernecotech.cylinderseal.feature.pay

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.modernecotech.cylinderseal.core.common.MoneyFormat
import com.modernecotech.cylinderseal.core.common.hexToBytes
import dagger.hilt.android.lifecycle.HiltViewModel
import javax.inject.Inject
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch

data class PayState(
    val recipientHex: String = "",
    val amountInput: String = "",
    val memo: String = "",
    val channel: PaymentBuilder.Channel = PaymentBuilder.Channel.NFC,
    val qrPayload: String? = null,
    val transactionId: String? = null,
    val error: String? = null,
    val busy: Boolean = false,
)

@HiltViewModel
class PayViewModel @Inject constructor(
    private val builder: PaymentBuilder,
) : ViewModel() {
    private val _state = MutableStateFlow(PayState())
    val state: StateFlow<PayState> = _state

    fun setRecipient(v: String) = update { it.copy(recipientHex = v.trim()) }
    fun setAmount(v: String) = update { it.copy(amountInput = v) }
    fun setMemo(v: String) = update { it.copy(memo = v) }
    fun setChannel(c: PaymentBuilder.Channel) = update { it.copy(channel = c) }

    fun submit() {
        val s = _state.value
        val amount = MoneyFormat.parseOwcString(s.amountInput) ?: run {
            update { it.copy(error = "Enter a valid amount") }
            return
        }
        val pk = runCatching { s.recipientHex.hexToBytes() }.getOrNull()
        if (pk == null || pk.size != 32) {
            update { it.copy(error = "Invalid recipient public key") }
            return
        }

        viewModelScope.launch {
            update { it.copy(busy = true, error = null) }
            try {
                val built = builder.build(
                    recipientPublicKey = pk,
                    amountMicroOwc = amount,
                    currency = "IQD",
                    channel = s.channel,
                    memo = s.memo,
                    latitude = 0.0,
                    longitude = 0.0,
                    locationAccuracyMeters = 0,
                )
                update {
                    it.copy(
                        busy = false,
                        qrPayload = built.qrPayload,
                        transactionId = built.transactionId,
                    )
                }
            } catch (t: Throwable) {
                update { it.copy(busy = false, error = t.message ?: "Payment build failed") }
            }
        }
    }

    fun reset() {
        _state.value = PayState()
    }

    private inline fun update(transform: (PayState) -> PayState) {
        _state.value = transform(_state.value)
    }
}
