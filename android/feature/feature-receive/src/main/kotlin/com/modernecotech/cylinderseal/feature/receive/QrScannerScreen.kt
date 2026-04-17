package com.modernecotech.cylinderseal.feature.receive

import android.Manifest
import androidx.camera.core.CameraSelector
import androidx.camera.core.ImageAnalysis
import androidx.camera.core.Preview
import androidx.camera.lifecycle.ProcessCameraProvider
import androidx.camera.view.PreviewView
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.compose.ui.viewinterop.AndroidView
import androidx.core.content.ContextCompat
import com.google.accompanist.permissions.ExperimentalPermissionsApi
import com.google.accompanist.permissions.isGranted
import com.google.accompanist.permissions.rememberPermissionState
import com.google.mlkit.vision.barcode.BarcodeScannerOptions
import com.google.mlkit.vision.barcode.BarcodeScanning
import com.google.mlkit.vision.barcode.common.Barcode
import com.google.mlkit.vision.common.InputImage
import java.util.concurrent.Executors

@OptIn(ExperimentalMaterial3Api::class, ExperimentalPermissionsApi::class)
@Composable
fun QrScannerScreen(
    onScanned: (String) -> Unit,
) {
    val permission = rememberPermissionState(Manifest.permission.CAMERA)
    LaunchedEffect(Unit) {
        if (!permission.status.isGranted) permission.launchPermissionRequest()
    }

    Scaffold(topBar = { TopAppBar(title = { Text("Scan QR") }) }) { padding ->
        Box(Modifier.fillMaxSize().padding(padding)) {
            if (!permission.status.isGranted) {
                Text("Camera permission required")
                return@Box
            }
            val ctx = LocalContext.current
            val lifecycleOwner = LocalLifecycleOwner.current
            val executor = remember { Executors.newSingleThreadExecutor() }
            val previewView = remember {
                PreviewView(ctx).apply {
                    scaleType = PreviewView.ScaleType.FILL_CENTER
                }
            }

            AndroidView(
                modifier = Modifier.fillMaxSize(),
                factory = { previewView },
                update = { view ->
                    val future = ProcessCameraProvider.getInstance(ctx)
                    future.addListener({
                        val provider = future.get()
                        val preview = Preview.Builder().build().apply {
                            setSurfaceProvider(view.surfaceProvider)
                        }
                        val analyzer = ImageAnalysis.Builder()
                            .setBackpressureStrategy(ImageAnalysis.STRATEGY_KEEP_ONLY_LATEST)
                            .build()

                        val scanner = BarcodeScanning.getClient(
                            BarcodeScannerOptions.Builder()
                                .setBarcodeFormats(Barcode.FORMAT_QR_CODE)
                                .build(),
                        )
                        analyzer.setAnalyzer(executor) { proxy ->
                            val mediaImage = proxy.image
                            if (mediaImage != null) {
                                val image = InputImage.fromMediaImage(
                                    mediaImage,
                                    proxy.imageInfo.rotationDegrees,
                                )
                                scanner.process(image)
                                    .addOnSuccessListener { barcodes ->
                                        val raw = barcodes.firstOrNull()?.rawValue
                                        if (raw != null) onScanned(raw)
                                    }
                                    .addOnCompleteListener { proxy.close() }
                            } else {
                                proxy.close()
                            }
                        }

                        try {
                            provider.unbindAll()
                            provider.bindToLifecycle(
                                lifecycleOwner,
                                CameraSelector.DEFAULT_BACK_CAMERA,
                                preview,
                                analyzer,
                            )
                        } catch (t: Throwable) {
                            timber.log.Timber.e(t, "camera bind failed")
                        }
                    }, ContextCompat.getMainExecutor(ctx))
                },
            )

            DisposableEffect(Unit) {
                onDispose { executor.shutdown() }
            }
        }
    }
}
