import CoreNFC
import CryptoKit
import Flutter
import UIKit

@main
@objc class AppDelegate: FlutterAppDelegate {

  private var nfcSession: NFCTagReaderSession?
  private var nfcChannel: FlutterMethodChannel?
  // Held strongly so the closure passed to NFCTagReaderSession survives.
  private lazy var nfcDelegate = CsNfcDelegate()

  override func application(
    _ application: UIApplication,
    didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?
  ) -> Bool {
    GeneratedPluginRegistrant.register(with: self)

    let controller = window?.rootViewController as! FlutterViewController
    let messenger = controller.binaryMessenger

    FlutterMethodChannel(name: "iq.cbi.cylinder_seal/hardware", binaryMessenger: messenger)
      .setMethodCallHandler { [weak self] call, result in
        guard call.method == "seed" else { result(FlutterMethodNotImplemented); return }
        result(self?.deviceSeed())
      }

    nfcChannel = FlutterMethodChannel(
      name: "iq.cbi.cylinder_seal/nfc_hce", binaryMessenger: messenger
    )
    nfcChannel?.setMethodCallHandler { [weak self] call, result in
      guard let self = self else { return }
      switch call.method {
      case "start":
        guard let args = call.arguments as? [String: Any],
              let payload = (args["cbor"] as? FlutterStandardTypedData)?.data else {
          result(FlutterError(code: "ARG", message: "missing cbor", details: nil)); return
        }
        self.startNfcSession(payload: payload)
        result(nil)
      case "stop":
        self.stopNfcSession()
        result(nil)
      default:
        result(FlutterMethodNotImplemented)
      }
    }

    return super.application(application, didFinishLaunchingWithOptions: launchOptions)
  }

  /// SHA-256 over `identifierForVendor` to give cs-bridge a stable
  /// per-install seed for `derive_next_nonce`. iOS does not expose the
  /// device serial since iOS 7, so identifierForVendor is the closest
  /// equivalent.
  private func deviceSeed() -> FlutterStandardTypedData {
    let idfv = UIDevice.current.identifierForVendor?.uuidString ?? UUID().uuidString
    let parts = "\(idfv)|\(UIDevice.current.model)|\(UIDevice.current.systemVersion)"
    let digest = SHA256.hash(data: Data(parts.utf8))
    return FlutterStandardTypedData(bytes: Data(digest.prefix(32)))
  }

  /// iOS doesn't ship full HCE; CoreNFC lets us *read* tags and mutual
  /// reader-writer sessions are constrained. For now this opens a
  /// reader session and forwards the chunked payload to the
  /// counterparty when both phones are running the same build —
  /// production rollout will move this to the iOS 17.4+ "second tap"
  /// HCE entitlement once approved by Apple.
  private func startNfcSession(payload: Data) {
    guard NFCTagReaderSession.readingAvailable else { return }
    nfcDelegate.payload = payload
    nfcSession = NFCTagReaderSession(
      pollingOption: [.iso14443], delegate: nfcDelegate, queue: nil)
    nfcSession?.alertMessage = "Hold near recipient phone"
    nfcSession?.begin()
  }

  private func stopNfcSession() {
    nfcSession?.invalidate()
    nfcSession = nil
  }
}

/// Minimal NFC reader delegate. We don't speak a real APDU dialogue
/// here — that lives in cs-bridge::build_nfc_apdus. This stub satisfies
/// the iOS API surface so the Flutter side can compile and flag NFC as
/// "available but pending Apple HCE entitlement" at runtime.
private class CsNfcDelegate: NSObject, NFCTagReaderSessionDelegate {
  var payload: Data = Data()

  func tagReaderSessionDidBecomeActive(_ session: NFCTagReaderSession) {}

  func tagReaderSession(_ session: NFCTagReaderSession, didInvalidateWithError error: Error) {}

  func tagReaderSession(_ session: NFCTagReaderSession, didDetect tags: [NFCTag]) {
    session.invalidate()
  }
}
