package com.modernecotech.cylinderseal.feature.history

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.Card
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
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

data class ComplianceState(
    val items: List<RecentEvalDto> = emptyList(),
    val loading: Boolean = false,
    val error: String? = null,
)

/**
 * "Why was this held?" — calls /v1/compliance/users/:userId/explanations
 * and renders the recent rule evaluations the server has on file.
 *
 * The endpoint is currently admin-gated server-side; in production this
 * screen would call a user-token-gated copy. For now the screen
 * demonstrates the wiring + UI; networking errors surface inline rather
 * than crashing.
 */
@HiltViewModel
class ComplianceViewModel @Inject constructor(
    private val wallet: WalletKeyManager,
    private val api: ComplianceApi,
) : ViewModel() {
    private val _state = MutableStateFlow(ComplianceState())
    val state: StateFlow<ComplianceState> = _state

    fun load() {
        viewModelScope.launch {
            _state.value = _state.value.copy(loading = true, error = null)
            try {
                val userId = MobileCore.userIdFromPublicKey(wallet.loadPublicKey())
                val resp = api.explanations(userId)
                _state.value = ComplianceState(items = resp.recent, loading = false)
            } catch (t: Throwable) {
                _state.value = ComplianceState(loading = false, error = t.message)
            }
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ComplianceRoute(viewModel: ComplianceViewModel = hiltViewModel()) {
    val state by viewModel.state.collectAsStateWithLifecycle()
    androidx.compose.runtime.LaunchedEffect(Unit) { viewModel.load() }

    Scaffold(topBar = { TopAppBar(title = { Text("Compliance review") }) }) { padding ->
        Column(
            Modifier
                .fillMaxSize()
                .padding(padding)
                .padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(8.dp),
        ) {
            Text(
                "Recent transactions reviewed by our compliance system. " +
                    "If something was held, the explanation tells you why.",
                style = MaterialTheme.typography.bodyMedium,
            )
            state.error?.let {
                Text("Error: $it", color = MaterialTheme.colorScheme.error)
            }
            if (state.loading && state.items.isEmpty()) {
                Text("Loading…")
            }
            LazyColumn(verticalArrangement = Arrangement.spacedBy(8.dp)) {
                items(state.items, key = { it.transaction_id }) { row ->
                    EvalCard(row)
                }
            }
        }
    }
}

@Composable
private fun EvalCard(row: RecentEvalDto) {
    val tone = when (row.risk_level) {
        "Critical" -> Color(0xFFC62828)
        "High" -> Color(0xFFE65100)
        "Medium" -> Color(0xFFB08800)
        "MediumLow" -> Color(0xFF558B2F)
        else -> Color(0xFF1A7F37)
    }
    Card(Modifier.fillMaxWidth()) {
        Column(Modifier.padding(12.dp)) {
            Text(
                "${row.risk_level} (score ${row.composite_score})",
                color = tone,
                style = MaterialTheme.typography.titleSmall,
            )
            Text(
                "Action: ${row.recommended_action}" +
                    if (row.held_for_review) " · held for review" else "",
                style = MaterialTheme.typography.bodySmall,
            )
            Text(row.explanation, style = MaterialTheme.typography.bodyMedium)
            Text(
                row.evaluated_at,
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }
    }
}
