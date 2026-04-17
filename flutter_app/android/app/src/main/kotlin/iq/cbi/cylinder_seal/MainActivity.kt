package iq.cbi.cylinder_seal

import android.os.Build
import android.provider.Settings
import io.flutter.embedding.android.FlutterActivity
import io.flutter.embedding.engine.FlutterEngine
import io.flutter.plugin.common.MethodChannel
import java.security.MessageDigest

class MainActivity : FlutterActivity() {

    override fun configureFlutterEngine(flutterEngine: FlutterEngine) {
        super.configureFlutterEngine(flutterEngine)

        MethodChannel(flutterEngine.dartExecutor.binaryMessenger, HARDWARE_CHANNEL)
            .setMethodCallHandler { call, result ->
                when (call.method) {
                    "seed" -> result.success(deviceSeed())
                    else -> result.notImplemented()
                }
            }

        MethodChannel(flutterEngine.dartExecutor.binaryMessenger, NFC_HCE_CHANNEL)
            .setMethodCallHandler { call, result ->
                when (call.method) {
                    "start" -> {
                        val cbor = call.argument<ByteArray>("cbor")
                        if (cbor == null) {
                            result.error("ARG", "missing cbor", null)
                        } else {
                            CsHostApduService.armPayload(cbor)
                            result.success(null)
                        }
                    }
                    "stop" -> {
                        CsHostApduService.disarm()
                        result.success(null)
                    }
                    else -> result.notImplemented()
                }
            }
    }

    /**
     * BLAKE2b-equivalent isn't in the Android SDK, so we hash with SHA-256
     * and truncate to 32 bytes — fine for the per-install pseudo-id whose
     * only purpose is to feed cs-bridge::derive_next_nonce as `hardware_seed`.
     * The Rust side then mixes it into the deterministic-nonce HKDF.
     */
    private fun deviceSeed(): ByteArray {
        val androidId = Settings.Secure.getString(contentResolver, Settings.Secure.ANDROID_ID) ?: ""
        val parts = listOf(
            androidId,
            Build.MANUFACTURER,
            Build.MODEL,
            Build.FINGERPRINT,
            Build.HARDWARE,
        ).joinToString("|")
        val digest = MessageDigest.getInstance("SHA-256").digest(parts.toByteArray())
        return digest.copyOf(32)
    }

    companion object {
        const val HARDWARE_CHANNEL = "iq.cbi.cylinder_seal/hardware"
        const val NFC_HCE_CHANNEL = "iq.cbi.cylinder_seal/nfc_hce"
    }
}
