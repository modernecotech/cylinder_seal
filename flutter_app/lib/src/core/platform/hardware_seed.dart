import 'dart:math';

import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';

/// Returns a stable per-install seed used as `hardware_seed` when deriving
/// nonces in `cs-bridge::derive_next_nonce`. Native side hashes its
/// platform IDs (Android `Build.SERIAL` / `ANDROID_ID`, iOS
/// `identifierForVendor`) and returns 32 bytes. On web there is no
/// hardware identity so we fall back to a CSPRNG seed persisted in
/// secure storage by the caller.
class HardwareSeed {
  HardwareSeed._();

  static const _channel = MethodChannel('iq.cbi.cylinder_seal/hardware');

  static Future<Uint8List> read() async {
    if (kIsWeb) {
      final r = Random.secure();
      return Uint8List.fromList(List.generate(32, (_) => r.nextInt(256)));
    }
    try {
      final raw = await _channel.invokeMethod<Uint8List>('seed');
      if (raw != null && raw.length == 32) return raw;
    } on MissingPluginException {
      // Native side not implemented yet — fall through to RNG fallback so
      // the app still runs in dev. Production rollout requires the
      // Kotlin/Swift handlers to be in place.
    } on PlatformException {
      // ditto
    }
    final r = Random.secure();
    return Uint8List.fromList(List.generate(32, (_) => r.nextInt(256)));
  }
}
