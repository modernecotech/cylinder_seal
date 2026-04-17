package com.modernecotech.cylinderseal.feature.onboarding

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.input.PasswordVisualTransformation
import androidx.compose.ui.unit.dp
import androidx.hilt.navigation.compose.hiltViewModel
import androidx.lifecycle.compose.collectAsStateWithLifecycle

@Composable
fun OnboardingRoute(
    onComplete: () -> Unit,
    viewModel: OnboardingViewModel = hiltViewModel(),
) {
    val state by viewModel.state.collectAsStateWithLifecycle()
    OnboardingScreen(
        state = state,
        onDisplayNameChange = viewModel::setDisplayName,
        onPhoneChange = viewModel::setPhoneNumber,
        onPinChange = viewModel::setPin,
        onPinConfirmChange = viewModel::setPinConfirm,
        onNext = viewModel::next,
        onComplete = onComplete,
    )
}

@Composable
fun OnboardingScreen(
    state: OnboardingState,
    onDisplayNameChange: (String) -> Unit,
    onPhoneChange: (String) -> Unit,
    onPinChange: (String) -> Unit,
    onPinConfirmChange: (String) -> Unit,
    onNext: () -> Unit,
    onComplete: () -> Unit,
) {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(24.dp),
        verticalArrangement = Arrangement.spacedBy(16.dp),
    ) {
        when (state.step) {
            OnboardingState.Step.Welcome -> {
                Text("Digital Iraqi Dinar", style = MaterialTheme.typography.headlineMedium)
                Text(
                    "Your phone is now your wallet. Zero fees, works offline, secured by " +
                        "hardware cryptography.",
                    style = MaterialTheme.typography.bodyLarge,
                )
                Spacer(Modifier.height(24.dp))
                Button(onClick = onNext, modifier = Modifier.fillMaxWidth()) {
                    Text("Get started")
                }
            }

            OnboardingState.Step.Profile -> {
                Text("Your profile", style = MaterialTheme.typography.titleLarge)
                OutlinedTextField(
                    value = state.displayName,
                    onValueChange = onDisplayNameChange,
                    label = { Text("Full name") },
                    singleLine = true,
                    modifier = Modifier.fillMaxWidth(),
                )
                OutlinedTextField(
                    value = state.phoneNumber,
                    onValueChange = onPhoneChange,
                    label = { Text("Phone (optional)") },
                    singleLine = true,
                    modifier = Modifier.fillMaxWidth(),
                )
                state.errorMessage?.let { ErrorText(it) }
                Button(onClick = onNext, modifier = Modifier.fillMaxWidth()) {
                    Text("Continue")
                }
            }

            OnboardingState.Step.SetPin -> {
                Text("Set a PIN", style = MaterialTheme.typography.titleLarge)
                Text(
                    "A 4-6 digit PIN protects every payment.",
                    style = MaterialTheme.typography.bodyMedium,
                )
                OutlinedTextField(
                    value = state.pin,
                    onValueChange = onPinChange,
                    label = { Text("PIN") },
                    visualTransformation = PasswordVisualTransformation(),
                    singleLine = true,
                    modifier = Modifier.fillMaxWidth(),
                )
                OutlinedTextField(
                    value = state.pinConfirm,
                    onValueChange = onPinConfirmChange,
                    label = { Text("Confirm PIN") },
                    visualTransformation = PasswordVisualTransformation(),
                    singleLine = true,
                    modifier = Modifier.fillMaxWidth(),
                )
                state.errorMessage?.let { ErrorText(it) }
                Button(onClick = onNext, modifier = Modifier.fillMaxWidth()) {
                    Text("Create wallet")
                }
            }

            OnboardingState.Step.GenerateKeys -> {
                Column(
                    modifier = Modifier.fillMaxSize(),
                    horizontalAlignment = Alignment.CenterHorizontally,
                    verticalArrangement = Arrangement.Center,
                ) {
                    CircularProgressIndicator()
                    Spacer(Modifier.height(16.dp))
                    Text("Generating hardware-backed keypair…")
                }
            }

            OnboardingState.Step.Done -> {
                Text("You're all set!", style = MaterialTheme.typography.headlineMedium)
                state.publicKeyHex?.let {
                    Text(
                        "Your wallet public key (first 12 chars): ${it.take(12)}…",
                        style = MaterialTheme.typography.bodyMedium,
                    )
                }
                Spacer(Modifier.height(24.dp))
                Button(onClick = onComplete, modifier = Modifier.fillMaxWidth()) {
                    Text("Open my wallet")
                }
            }
        }
    }
}

@Composable
private fun ErrorText(msg: String) {
    Text(msg, color = MaterialTheme.colorScheme.error)
}
