package com.modernecotech.cylinderseal.feature.settings

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.hilt.navigation.compose.hiltViewModel
import androidx.lifecycle.ViewModel
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewModelScope
import com.modernecotech.cylinderseal.core.datastore.UserPreferences
import dagger.hilt.android.lifecycle.HiltViewModel
import javax.inject.Inject
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.launch

data class SettingsState(
    val host: String = "",
    val port: String = "",
    val lastSync: Long = 0L,
)

@HiltViewModel
class SettingsViewModel @Inject constructor(
    private val prefs: UserPreferences,
) : ViewModel() {
    private val _state = MutableStateFlow(SettingsState())
    val state: StateFlow<SettingsState> = _state

    init {
        viewModelScope.launch {
            _state.value = SettingsState(
                host = prefs.superpeerHost.first(),
                port = prefs.superpeerPort.first().toString(),
                lastSync = prefs.lastSyncAt.first(),
            )
        }
    }

    fun setHost(v: String) { _state.value = _state.value.copy(host = v) }
    fun setPort(v: String) { _state.value = _state.value.copy(port = v.filter(Char::isDigit)) }

    fun save() {
        viewModelScope.launch {
            val port = _state.value.port.toIntOrNull() ?: return@launch
            prefs.setSuperpeer(_state.value.host, port)
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun SettingsRoute(viewModel: SettingsViewModel = hiltViewModel()) {
    val state by viewModel.state.collectAsStateWithLifecycle()
    Scaffold(topBar = { TopAppBar(title = { Text("Settings") }) }) { padding ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(padding)
                .padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            Text("Super-peer endpoint", style = MaterialTheme.typography.titleMedium)
            OutlinedTextField(
                value = state.host,
                onValueChange = viewModel::setHost,
                label = { Text("Host") },
                singleLine = true,
                modifier = Modifier.fillMaxWidth(),
            )
            OutlinedTextField(
                value = state.port,
                onValueChange = viewModel::setPort,
                label = { Text("Port") },
                singleLine = true,
                modifier = Modifier.fillMaxWidth(),
            )
            Button(onClick = viewModel::save, modifier = Modifier.fillMaxWidth()) {
                Text("Save")
            }
            if (state.lastSync > 0) {
                Text(
                    "Last sync: ${java.text.SimpleDateFormat("yyyy-MM-dd HH:mm").format(java.util.Date(state.lastSync))}",
                    style = MaterialTheme.typography.bodySmall,
                )
            }
        }
    }
}
