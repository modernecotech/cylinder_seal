package com.modernecotech.cylinderseal.feature.receive

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.modernecotech.cylinderseal.core.common.toHex
import com.modernecotech.cylinderseal.core.cryptography.WalletKeyManager
import com.modernecotech.cylinderseal.core.ffi.MobileCore
import com.modernecotech.cylinderseal.feature.receive.ingest.IncomingPaymentIngestor
import dagger.hilt.android.lifecycle.HiltViewModel
import javax.inject.Inject
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch

data class ReceiveState(
    val publicKeyHex: String = "",
    val userId: String = "",
    val lastIngestStatus: IngestStatus = IngestStatus.Idle,
) {
    enum class IngestStatus { Idle, Accepted, Rejected }
}

@HiltViewModel
class ReceiveViewModel @Inject constructor(
    private val wallet: WalletKeyManager,
    private val ingestor: IncomingPaymentIngestor,
) : ViewModel() {
    private val _state = MutableStateFlow(ReceiveState())
    val state: StateFlow<ReceiveState> = _state

    init {
        viewModelScope.launch {
            val pk = wallet.loadPublicKey()
            _state.value = _state.value.copy(
                publicKeyHex = pk.toHex(),
                userId = MobileCore.userIdFromPublicKey(pk),
            )
        }
    }

    fun onQrScanned(raw: String) {
        viewModelScope.launch {
            val cbor = runCatching { MobileCore.decodeQrPayload(raw) }.getOrNull()
            if (cbor == null) {
                _state.value = _state.value.copy(
                    lastIngestStatus = ReceiveState.IngestStatus.Rejected,
                )
                return@launch
            }
            val accepted = ingestor.onPayload(cbor)
            _state.value = _state.value.copy(
                lastIngestStatus = if (accepted) ReceiveState.IngestStatus.Accepted
                else ReceiveState.IngestStatus.Rejected,
            )
        }
    }
}
