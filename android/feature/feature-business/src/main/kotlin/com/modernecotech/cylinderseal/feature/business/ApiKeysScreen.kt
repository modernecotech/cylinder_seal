package com.modernecotech.cylinderseal.feature.business

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Button
import androidx.compose.material3.Card
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.hilt.navigation.compose.hiltViewModel
import androidx.lifecycle.ViewModel
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewModelScope
import com.modernecotech.cylinderseal.core.cryptography.WalletKeyManager
import com.modernecotech.cylinderseal.core.ffi.MobileCore
import dagger.hilt.android.lifecycle.HiltViewModel
import javax.inject.Inject
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch

data class ApiKeysState(
    val keys: List<ApiKeyListItem> = emptyList(),
    val newKeySecret: String? = null, // shown once
    val newKeyLabel: String = "",
    val busy: Boolean = false,
    val error: String? = null,
)

@HiltViewModel
class ApiKeysViewModel @Inject constructor(
    private val wallet: WalletKeyManager,
    private val api: BusinessApi,
) : ViewModel() {
    private val _state = MutableStateFlow(ApiKeysState())
    val state: StateFlow<ApiKeysState> = _state

    private suspend fun userId(): String =
        MobileCore.userIdFromPublicKey(wallet.loadPublicKey())

    fun refresh() {
        viewModelScope.launch {
            try {
                _state.value = _state.value.copy(keys = api.listApiKeys(userId()), error = null)
            } catch (t: Throwable) {
                _state.value = _state.value.copy(error = t.message)
            }
        }
    }

    fun setLabel(v: String) {
        _state.value = _state.value.copy(newKeyLabel = v)
    }

    fun issue() {
        viewModelScope.launch {
            _state.value = _state.value.copy(busy = true, error = null)
            try {
                val resp = api.issueApiKey(userId(), IssueKeyRequest(label = _state.value.newKeyLabel))
                _state.value = _state.value.copy(
                    busy = false,
                    newKeySecret = resp.secret,
                    newKeyLabel = "",
                )
                refresh()
            } catch (t: Throwable) {
                _state.value = _state.value.copy(busy = false, error = t.message)
            }
        }
    }

    fun dismissSecret() {
        _state.value = _state.value.copy(newKeySecret = null)
    }

    fun revoke(id: Long) {
        viewModelScope.launch {
            try {
                api.revokeApiKey(userId(), id)
                refresh()
            } catch (t: Throwable) {
                _state.value = _state.value.copy(error = t.message)
            }
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ApiKeysRoute(viewModel: ApiKeysViewModel = hiltViewModel()) {
    val state by viewModel.state.collectAsStateWithLifecycle()
    androidx.compose.runtime.LaunchedEffect(Unit) { viewModel.refresh() }

    Scaffold(topBar = { TopAppBar(title = { Text("API keys") }) }) { padding ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(padding)
                .padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            Text(
                "API keys let your server authenticate against the business API. Each key is shown exactly once — copy it before dismissing.",
                style = MaterialTheme.typography.bodyMedium,
            )

            Row(verticalAlignment = androidx.compose.ui.Alignment.CenterVertically) {
                OutlinedTextField(
                    value = state.newKeyLabel,
                    onValueChange = viewModel::setLabel,
                    label = { Text("Label (e.g. 'production-store-backend')") },
                    singleLine = true,
                    modifier = Modifier.weight(1f),
                )
                Spacer(Modifier.padding(4.dp))
                Button(onClick = viewModel::issue, enabled = !state.busy && state.newKeyLabel.isNotBlank()) {
                    Text("Issue")
                }
            }

            state.error?.let { Text(it, color = MaterialTheme.colorScheme.error) }

            LazyColumn(verticalArrangement = Arrangement.spacedBy(8.dp)) {
                items(state.keys, key = { it.id }) { key ->
                    Card(Modifier.fillMaxWidth()) {
                        Row(
                            Modifier.fillMaxWidth().padding(12.dp),
                            verticalAlignment = androidx.compose.ui.Alignment.CenterVertically,
                        ) {
                            Column(Modifier.weight(1f)) {
                                Text(key.label)
                                Text(
                                    "${key.key_prefix}… · created ${key.created_at}",
                                    style = MaterialTheme.typography.bodySmall,
                                )
                                if (key.revoked) {
                                    Text("Revoked", color = MaterialTheme.colorScheme.error, style = MaterialTheme.typography.bodySmall)
                                } else {
                                    key.last_used_at?.let {
                                        Text("Last used: $it", style = MaterialTheme.typography.bodySmall)
                                    }
                                }
                            }
                            if (!key.revoked) {
                                TextButton(onClick = { viewModel.revoke(key.id) }) { Text("Revoke") }
                            }
                        }
                    }
                }
            }
        }
    }

    state.newKeySecret?.let { secret ->
        AlertDialog(
            onDismissRequest = viewModel::dismissSecret,
            title = { Text("New API key") },
            text = {
                Column {
                    Text(
                        "This is the only time the secret will be shown. Copy it to a safe place now — it cannot be recovered later.",
                        style = MaterialTheme.typography.bodyMedium,
                    )
                    Spacer(Modifier.height(8.dp))
                    Text(secret, style = MaterialTheme.typography.bodyLarge)
                }
            },
            confirmButton = {
                Button(onClick = viewModel::dismissSecret) { Text("I've saved it") }
            },
        )
    }
}
