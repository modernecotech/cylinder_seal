package com.modernecotech.cylinderseal.feature.wallet

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowDownward
import androidx.compose.material.icons.filled.ArrowUpward
import androidx.compose.material.icons.filled.History
import androidx.compose.material.icons.filled.QrCodeScanner
import androidx.compose.material.icons.filled.Send
import androidx.compose.material.icons.filled.Settings
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.FilledTonalButton
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import androidx.hilt.navigation.compose.hiltViewModel
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import com.modernecotech.cylinderseal.core.common.MoneyFormat
import com.modernecotech.cylinderseal.core.database.TransactionEntity

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun WalletRoute(
    onSendClick: () -> Unit,
    onReceiveClick: () -> Unit,
    onScanClick: () -> Unit,
    onHistoryClick: () -> Unit,
    onSettingsClick: () -> Unit,
    viewModel: WalletViewModel = hiltViewModel(),
) {
    val state by viewModel.uiState.collectAsStateWithLifecycle()
    Scaffold(
        topBar = {
            TopAppBar(title = { Text("Digital Iraqi Dinar") })
        },
    ) { padding ->
        WalletContent(
            state = state,
            padding = padding,
            onSendClick = onSendClick,
            onReceiveClick = onReceiveClick,
            onScanClick = onScanClick,
            onHistoryClick = onHistoryClick,
            onSettingsClick = onSettingsClick,
        )
    }
}

@Composable
private fun WalletContent(
    state: WalletUiState,
    padding: PaddingValues,
    onSendClick: () -> Unit,
    onReceiveClick: () -> Unit,
    onScanClick: () -> Unit,
    onHistoryClick: () -> Unit,
    onSettingsClick: () -> Unit,
) {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(padding)
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(16.dp),
    ) {
        BalanceCard(state)
        QuickActions(
            onSendClick = onSendClick,
            onReceiveClick = onReceiveClick,
            onScanClick = onScanClick,
            onSettingsClick = onSettingsClick,
        )
        Text("Recent", style = MaterialTheme.typography.titleMedium)
        if (state.recent.isEmpty()) {
            Text(
                "Nothing yet — tap Receive to show your QR code, or Send to pay.",
                style = MaterialTheme.typography.bodyMedium,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        } else {
            LazyColumn(verticalArrangement = Arrangement.spacedBy(8.dp)) {
                items(state.recent, key = { it.transactionId }) { tx ->
                    TransactionRow(tx)
                }
            }
        }

        Spacer(Modifier.height(8.dp))
        FilledTonalButton(onClick = onHistoryClick, modifier = Modifier.fillMaxWidth()) {
            Icon(Icons.Filled.History, null)
            Spacer(Modifier.padding(4.dp))
            Text("Full history")
        }
    }
}

@Composable
private fun BalanceCard(state: WalletUiState) {
    Card(
        modifier = Modifier.fillMaxWidth(),
        colors = CardDefaults.cardColors(containerColor = MaterialTheme.colorScheme.primaryContainer),
    ) {
        Column(Modifier.padding(20.dp)) {
            Text(state.displayName.ifBlank { "Wallet" }, style = MaterialTheme.typography.titleSmall)
            Spacer(Modifier.height(8.dp))
            Text(
                MoneyFormat.format(state.balanceMicroOwc),
                style = MaterialTheme.typography.headlineLarge,
            )
            if (state.pendingCount > 0) {
                Spacer(Modifier.height(4.dp))
                Text(
                    "${state.pendingCount} pending sync",
                    style = MaterialTheme.typography.bodySmall,
                )
            }
        }
    }
}

@Composable
private fun QuickActions(
    onSendClick: () -> Unit,
    onReceiveClick: () -> Unit,
    onScanClick: () -> Unit,
    onSettingsClick: () -> Unit,
) {
    Row(
        modifier = Modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.spacedBy(12.dp),
    ) {
        QuickAction("Send", Icons.Filled.Send, Modifier.weight(1f), onSendClick)
        QuickAction("Receive", Icons.Filled.ArrowDownward, Modifier.weight(1f), onReceiveClick)
        QuickAction("Scan", Icons.Filled.QrCodeScanner, Modifier.weight(1f), onScanClick)
        QuickAction("Settings", Icons.Filled.Settings, Modifier.weight(1f), onSettingsClick)
    }
}

@Composable
private fun QuickAction(
    label: String,
    icon: androidx.compose.ui.graphics.vector.ImageVector,
    modifier: Modifier,
    onClick: () -> Unit,
) {
    FilledTonalButton(onClick = onClick, modifier = modifier) {
        Column(horizontalAlignment = Alignment.CenterHorizontally) {
            Icon(icon, null)
            Spacer(Modifier.height(4.dp))
            Text(label)
        }
    }
}

@Composable
private fun TransactionRow(tx: TransactionEntity) {
    Card(Modifier.fillMaxWidth()) {
        Row(
            Modifier
                .fillMaxWidth()
                .padding(12.dp),
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.SpaceBetween,
        ) {
            Row(verticalAlignment = Alignment.CenterVertically) {
                Icon(
                    if (tx.direction == "INCOMING") Icons.Filled.ArrowDownward
                    else Icons.Filled.ArrowUpward,
                    null,
                    tint = if (tx.direction == "INCOMING") Color(0xFF2E7D32)
                    else MaterialTheme.colorScheme.error,
                )
                Spacer(Modifier.padding(4.dp))
                Column {
                    Text(tx.counterpartyName ?: tx.counterpartyPublicKeyHex.take(10))
                    Text(
                        "${tx.channel} · ${tx.syncStatus}",
                        style = MaterialTheme.typography.bodySmall,
                    )
                }
            }
            Text(
                (if (tx.direction == "INCOMING") "+" else "-") +
                    MoneyFormat.format(tx.amountMicroOwc, tx.currency),
            )
        }
    }
}
