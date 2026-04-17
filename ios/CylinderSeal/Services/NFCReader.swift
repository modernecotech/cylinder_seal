import CoreNFC
import Foundation

/// Reader-side NFC support.
///
/// iOS does NOT expose Host Card Emulation (Android's HCE equivalent) to
/// third-party apps. Reader-side though, we can talk to any ISO 14443-4
/// tag — which is exactly what Android's `CylinderSealApduService` emulates.
/// So the flow is: iOS initiates a tag-reader session, finds the CylinderSeal
/// AID, issues PROPOSE-style APDUs to pull the signed CBOR, and hands the
/// payload to the ingestor.
///
/// The sender side (iPhone → another device) does **not** work over NFC
/// on iOS; iPhones must fall back to BLE or QR to transmit a signed
/// transaction.
@MainActor
final class NFCReader: NSObject, ObservableObject {
    @Published private(set) var isAvailable = NFCTagReaderSession.readingAvailable
    @Published private(set) var lastError: String?

    private var session: NFCTagReaderSession?
    private let ingestor: IncomingPaymentIngestor

    // Matches `crates/cs-mobile-core/src/wire.rs` / Android APDU service.
    private static let csAID = Data([0xF0, 0xCB, 0xCD, 0x01, 0x00])

    init(ingestor: IncomingPaymentIngestor) {
        self.ingestor = ingestor
        super.init()
    }

    func beginSession() {
        guard NFCTagReaderSession.readingAvailable else {
            lastError = "NFC not available on this device"
            return
        }
        let session = NFCTagReaderSession(
            pollingOption: [.iso14443],
            delegate: self,
            queue: nil
        )
        session?.alertMessage = "Hold device near the sender's phone"
        session?.begin()
        self.session = session
    }
}

extension NFCReader: NFCTagReaderSessionDelegate {
    nonisolated func tagReaderSessionDidBecomeActive(_ session: NFCTagReaderSession) {}

    nonisolated func tagReaderSession(_ session: NFCTagReaderSession, didInvalidateWithError error: Error) {
        Task { @MainActor in
            self.lastError = error.localizedDescription
        }
    }

    nonisolated func tagReaderSession(_ session: NFCTagReaderSession, didDetect tags: [NFCTag]) {
        guard case let .iso7816(iso) = tags.first else {
            session.invalidate(errorMessage: "Unsupported tag")
            return
        }
        session.connect(to: tags.first!) { [weak self] error in
            guard let self else { return }
            if let error {
                session.invalidate(errorMessage: error.localizedDescription)
                return
            }
            Task { await self.pullPayload(from: iso, session: session) }
        }
    }

    nonisolated private func pullPayload(from iso: NFCISO7816Tag, session: NFCTagReaderSession) async {
        // 1. SELECT AID.
        let select = NFCISO7816APDU(
            instructionClass: 0x00,
            instructionCode: 0xA4,
            p1Parameter: 0x04,
            p2Parameter: 0x00,
            data: Self.csAID,
            expectedResponseLength: 256
        )
        do {
            let (_, sw1, sw2) = try await iso.sendCommand(apdu: select)
            guard sw1 == 0x90, sw2 == 0x00 else {
                session.invalidate(errorMessage: "SELECT failed")
                return
            }

            // 2. GET-DATA loop.
            var payload = Data()
            let get = NFCISO7816APDU(
                instructionClass: 0x80,
                instructionCode: 0x20,
                p1Parameter: 0x00,
                p2Parameter: 0x00,
                data: Data(),
                expectedResponseLength: 256
            )
            while true {
                let (data, sw1, sw2) = try await iso.sendCommand(apdu: get)
                if sw1 == 0x90 && sw2 == 0x00 {
                    payload.append(data)
                    if data.count < 253 { break } // short-read sentinel
                } else if sw1 == 0x6A && sw2 == 0x82 {
                    break
                } else {
                    session.invalidate(errorMessage: "Unexpected status word")
                    return
                }
            }

            await MainActor.run {
                _ = self.ingestor.onPayload(payload, transport: .nfc)
            }
            session.alertMessage = "Payment received"
            session.invalidate()
        } catch {
            session.invalidate(errorMessage: error.localizedDescription)
        }
    }
}
