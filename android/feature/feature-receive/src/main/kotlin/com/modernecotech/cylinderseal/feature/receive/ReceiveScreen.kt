package com.modernecotech.cylinderseal.feature.receive

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.material3.Button
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.MaterialTheme
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
import com.google.zxing.BarcodeFormat
import com.google.zxing.EncodeHintType
import com.google.zxing.qrcode.QRCodeWriter
import com.google.zxing.qrcode.decoder.ErrorCorrectionLevel

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ReceiveRoute(
    onScanClick: () -> Unit,
    onBack: () -> Unit,
    viewModel: ReceiveViewModel = hiltViewModel(),
) {
    val state by viewModel.state.collectAsStateWithLifecycle()
    Scaffold(topBar = { TopAppBar(title = { Text("Receive") }) }) { padding ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(padding)
                .padding(16.dp),
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            Text("Show this code to the payer", style = MaterialTheme.typography.titleMedium)
            val qrBmp = remember(state.publicKeyHex) {
                renderWalletQr(state.publicKeyHex)
            }
            if (state.publicKeyHex.isNotEmpty()) {
                Image(
                    bitmap = qrBmp.asImageBitmap(),
                    contentDescription = "Wallet QR",
                    modifier = Modifier.size(256.dp),
                )
                Text("PK ${state.publicKeyHex.take(12)}…", style = MaterialTheme.typography.bodySmall)
                Text("User ${state.userId.take(8)}…", style = MaterialTheme.typography.bodySmall)
            }
            Spacer(Modifier.height(16.dp))
            Text(
                "Tap your phone to theirs for NFC, or use the Scan button if they show a code.",
                style = MaterialTheme.typography.bodyMedium,
            )
            Button(onClick = onScanClick, modifier = Modifier.fillMaxWidth()) {
                Text("Scan sender's QR")
            }
            state.lastIngestStatus.takeIf { it != ReceiveState.IngestStatus.Idle }?.let { status ->
                Text(
                    if (status == ReceiveState.IngestStatus.Accepted) "Payment received."
                    else "Payment rejected (invalid signature).",
                    color = if (status == ReceiveState.IngestStatus.Accepted)
                        MaterialTheme.colorScheme.primary
                    else MaterialTheme.colorScheme.error,
                )
            }
            Button(onClick = onBack, modifier = Modifier.fillMaxWidth()) {
                Text("Done")
            }
        }
    }
}

private fun renderWalletQr(payloadHex: String): android.graphics.Bitmap {
    val payload = "CS1:PK:$payloadHex"
    val hints = mapOf(
        EncodeHintType.ERROR_CORRECTION to ErrorCorrectionLevel.M,
        EncodeHintType.MARGIN to 2,
    )
    val matrix = QRCodeWriter().encode(payload, BarcodeFormat.QR_CODE, 512, 512, hints)
    val bmp = android.graphics.Bitmap.createBitmap(512, 512, android.graphics.Bitmap.Config.ARGB_8888)
    for (x in 0 until 512) for (y in 0 until 512) {
        bmp.setPixel(
            x, y,
            if (matrix[x, y]) android.graphics.Color.BLACK else android.graphics.Color.WHITE,
        )
    }
    return bmp
}
