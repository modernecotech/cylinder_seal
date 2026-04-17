import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';

/// Bridge to the platform-specific NFC HCE / CoreNFC handler.
///
/// - Android: a Kotlin `HostApduService` registered in `AndroidManifest.xml`
///   listens for `SELECT AID F0 CB CD 01 00` and replies with the
///   chunked `propose` frames produced by `cs-bridge::build_nfc_apdus`.
/// - iOS: a `CoreNFC` reader session reads the same APDUs from the
///   counterparty acting as HCE — symmetrical to the Android side.
/// - Web: not supported; calls become no-ops so the same code compiles.
class NfcHce {
  NfcHce._();

  static const _channel = MethodChannel('iq.cbi.cylinder_seal/nfc_hce');

  /// Hand the signed CBOR off to the native side, which will respond to
  /// the next reader poll with the matching APDUs.
  static Future<void> startSession(List<int> cbor) async {
    if (kIsWeb) return;
    try {
      await _channel.invokeMethod('start', {
        'cbor': Uint8List.fromList(cbor),
      });
    } on MissingPluginException {
      // Native side is wired up in the Flutter Android/iOS modules but
      // may not be present in dev builds — surface as a clear error.
      throw StateError('NFC HCE plugin not registered on this platform.');
    }
  }

  static Future<void> endSession() async {
    if (kIsWeb) return;
    try {
      await _channel.invokeMethod('stop');
    } on MissingPluginException {
      // Best-effort.
    } on PlatformException {
      // Best-effort.
    }
  }
}
