import Foundation

@MainActor
final class ReceiveViewModel: ObservableObject {
    @Published var publicKeyHex: String = ""
    @Published var userId: String = ""
    @Published var showScanner = false

    private let wallet: WalletKeyManager
    private let ingestor: IncomingPaymentIngestor

    init(wallet: WalletKeyManager, ingestor: IncomingPaymentIngestor) {
        self.wallet = wallet
        self.ingestor = ingestor
    }

    func onAppear() {
        do {
            let pk = try wallet.loadPublicKey()
            publicKeyHex = pk.toHex()
            userId = (try? MobileCore.userIdFromPublicKey(pk)) ?? ""
        } catch {
            publicKeyHex = ""
        }
    }

    func handleScan(_ raw: String) {
        showScanner = false
        // Accept signed payment payloads (CS1:<hex>). Ignore our own
        // wallet-address QRs (CS1:PK:<hex>) and payment-request QRs
        // (CS1:REQ:<hex>).
        guard let rest = stripPrefix(raw),
              !rest.hasPrefix("PK:") && !rest.hasPrefix("REQ:"),
              let cbor = Data(hex: rest)
        else { return }
        _ = ingestor.onPayload(cbor, transport: .online)
    }

    private func stripPrefix(_ raw: String) -> String? {
        raw.hasPrefix("CS1:") ? String(raw.dropFirst(4)) : nil
    }
}
