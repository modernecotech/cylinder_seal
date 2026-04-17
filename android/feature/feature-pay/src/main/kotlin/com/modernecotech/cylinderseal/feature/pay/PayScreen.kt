package com.modernecotech.cylinderseal.feature.pay

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.selection.selectable
import androidx.compose.material3.Button
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.RadioButton
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.asImageBitmap
import androidx.compose.ui.unit.dp
import androidx.hilt.navigation.compose.hiltViewModel
import androidx.lifecycle.compose.collectAsStateWithLifecycle

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun PayRoute(
    onBack: () -> Unit,
    viewModel: PayViewModel = hiltViewModel(),
) {
    val state by viewModel.state.collectAsStateWithLifecycle()
    Scaffold(topBar = { TopAppBar(title = { Text("Send") }) }) { padding ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(padding)
                .padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            OutlinedTextField(
                value = state.recipientHex,
                onValueChange = viewModel::setRecipient,
                label = { Text("Recipient public key (hex)") },
                singleLine = true,
                modifier = Modifier.fillMaxWidth(),
            )
            OutlinedTextField(
                value = state.amountInput,
                onValueChange = viewModel::setAmount,
                label = { Text("Amount (IQD)") },
                singleLine = true,
                modifier = Modifier.fillMaxWidth(),
            )
            OutlinedTextField(
                value = state.memo,
                onValueChange = viewModel::setMemo,
                label = { Text("Memo (optional)") },
                singleLine = true,
                modifier = Modifier.fillMaxWidth(),
            )
            ChannelSelector(state.channel, viewModel::setChannel)
            state.error?.let {
                Text(it, color = MaterialTheme.colorScheme.error)
            }
            Button(
                onClick = viewModel::submit,
                enabled = !state.busy,
                modifier = Modifier.fillMaxWidth(),
            ) {
                if (state.busy) CircularProgressIndicator() else Text("Sign & send")
            }
            state.qrPayload?.let { payload ->
                Spacer(Modifier.height(24.dp))
                Text("Show this code to the recipient:")
                val bmp = remember(payload) { QrRenderer.render(payload) }
                Image(
                    bitmap = bmp.asImageBitmap(),
                    contentDescription = "Payment QR",
                    modifier = Modifier
                        .size(256.dp)
                        .padding(top = 8.dp),
                )
                state.transactionId?.let {
                    Text("Tx id: ${it.take(12)}…", style = MaterialTheme.typography.bodySmall)
                }
                Button(onClick = onBack, modifier = Modifier.fillMaxWidth()) {
                    Text("Done")
                }
            }
        }
    }
}

@Composable
private fun ChannelSelector(
    current: PaymentBuilder.Channel,
    onChange: (PaymentBuilder.Channel) -> Unit,
) {
    Column {
        Text("Transport", style = MaterialTheme.typography.labelLarge)
        PaymentBuilder.Channel.values().forEach { ch ->
            androidx.compose.foundation.layout.Row(
                verticalAlignment = Alignment.CenterVertically,
                modifier = Modifier
                    .fillMaxWidth()
                    .selectable(selected = current == ch, onClick = { onChange(ch) })
                    .padding(vertical = 4.dp),
            ) {
                RadioButton(selected = current == ch, onClick = { onChange(ch) })
                Text(ch.name)
            }
        }
    }
}
