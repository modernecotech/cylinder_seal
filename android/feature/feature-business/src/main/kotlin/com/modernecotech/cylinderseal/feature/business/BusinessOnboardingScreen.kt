package com.modernecotech.cylinderseal.feature.business

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
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
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.hilt.navigation.compose.hiltViewModel
import androidx.lifecycle.compose.collectAsStateWithLifecycle

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun BusinessOnboardingRoute(
    onDone: () -> Unit,
    viewModel: BusinessOnboardingViewModel = hiltViewModel(),
) {
    val state by viewModel.state.collectAsStateWithLifecycle()

    Scaffold(topBar = { TopAppBar(title = { Text("Register business") }) }) { padding ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(padding)
                .padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            if (state.submitted) {
                Text("Submitted", style = MaterialTheme.typography.headlineSmall)
                state.statusMessage?.let { Text(it, style = MaterialTheme.typography.bodyMedium) }
                Spacer(Modifier.height(12.dp))
                Button(onClick = onDone, modifier = Modifier.fillMaxWidth()) {
                    Text("Done")
                }
                return@Column
            }

            KindSelector(state.kind, viewModel::setKind)

            OutlinedTextField(
                value = state.legalName,
                onValueChange = viewModel::setLegalName,
                label = { Text("Legal name") },
                singleLine = true,
                modifier = Modifier.fillMaxWidth(),
            )
            OutlinedTextField(
                value = state.commercialRegistrationId,
                onValueChange = viewModel::setRegistration,
                label = { Text("Commercial registration (Sijel Tijari)") },
                singleLine = true,
                modifier = Modifier.fillMaxWidth(),
            )
            OutlinedTextField(
                value = state.taxId,
                onValueChange = viewModel::setTaxId,
                label = { Text("Tax ID") },
                singleLine = true,
                modifier = Modifier.fillMaxWidth(),
            )
            OutlinedTextField(
                value = state.industryCode,
                onValueChange = viewModel::setIndustry,
                label = { Text("Industry code (ISIC, e.g. 4711)") },
                singleLine = true,
                modifier = Modifier.fillMaxWidth(),
            )
            OutlinedTextField(
                value = state.contactEmail,
                onValueChange = viewModel::setEmail,
                label = { Text("Contact email") },
                singleLine = true,
                modifier = Modifier.fillMaxWidth(),
            )
            OutlinedTextField(
                value = state.registeredAddress,
                onValueChange = viewModel::setAddress,
                label = { Text("Registered address") },
                minLines = 2,
                modifier = Modifier.fillMaxWidth(),
            )

            state.errorMessage?.let {
                Text(it, color = MaterialTheme.colorScheme.error)
            }

            Button(
                onClick = viewModel::submit,
                enabled = !state.busy,
                modifier = Modifier.fillMaxWidth(),
            ) {
                if (state.busy) CircularProgressIndicator() else Text("Submit registration")
            }
            Text(
                "CBI ops verifies your commercial registration and tax ID against the national registry before activating the account. This usually takes 1-3 business days.",
                style = MaterialTheme.typography.bodySmall,
            )
        }
    }
}

@Composable
private fun KindSelector(current: BusinessKind, onChange: (BusinessKind) -> Unit) {
    Column {
        Text("Business type", style = MaterialTheme.typography.labelLarge)
        BusinessKind.values().forEach { k ->
            Row(
                verticalAlignment = Alignment.CenterVertically,
                modifier = Modifier
                    .fillMaxWidth()
                    .selectable(selected = current == k, onClick = { onChange(k) })
                    .padding(vertical = 4.dp),
            ) {
                RadioButton(selected = current == k, onClick = { onChange(k) })
                Column {
                    Text(
                        when (k) {
                            BusinessKind.POS -> "Physical point of sale"
                            BusinessKind.ELECTRONIC -> "Electronic / online commerce"
                        }
                    )
                    Text(
                        when (k) {
                            BusinessKind.POS -> "Shops, market stalls, services with a physical till."
                            BusinessKind.ELECTRONIC -> "E-commerce, B2B, SaaS. Includes API key access."
                        },
                        style = MaterialTheme.typography.bodySmall,
                    )
                }
            }
        }
    }
}
