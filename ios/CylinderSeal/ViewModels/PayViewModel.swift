import Foundation

@MainActor
final class PayViewModel: ObservableObject {
    @Published var recipientHex: String = ""
    @Published var amountInput: String = ""
    @Published var memo: String = ""
    @Published var channel: PaymentChannel = .ble
    @Published var qrPayload: String?
    @Published var transactionId: String?
    @Published var error: String?
    @Published var busy = false

    private let wallet: WalletKeyManager
    private let database: Database

    init(wallet: WalletKeyManager, database: Database) {
        self.wallet = wallet
        self.database = database
    }

    func submit() {
        error = nil
        guard let recipient = Data(hex: recipientHex.trimmingCharacters(in: .whitespaces)),
              recipient.count == 32
        else {
            error = "Invalid recipient public key"
            return
        }
        guard let amount = MoneyFormat.parse(amountInput), amount > 0 else {
            error = "Enter a valid amount"
            return
        }

        busy = true
        Task {
            defer { busy = false }
            do {
                let ownPk = try wallet.loadPublicKey()
                var privKey = try wallet.loadPrivateKey()
                defer { privKey.resetBytes(in: 0..<privKey.count) }

                let latestNonce = try database.latestNonce()
                let counter = (latestNonce?.counter ?? 0) + 1
                let prevNonce: Data
                if let latest = latestNonce, let decoded = Data(hex: latest.nonceHex) {
                    prevNonce = decoded
                } else {
                    prevNonce = Data(count: 32) // genesis
                }

                let seed = MobileCore.blake2b256(ownPk + Data([UInt8(channel.rawValue)]))
                let nextNonce = try MobileCore.deriveNextNonce(
                    prev: prevNonce,
                    hardwareSeed: seed,
                    counter: UInt64(counter)
                )

                let deviceId = try stableDeviceId()
                let input = MobileCore.TransactionInput(
                    fromPublicKey: ownPk,
                    toPublicKey: recipient,
                    amountMicroOwc: amount,
                    currencyContext: "IQD",
                    fxRateSnapshot: "1.0",
                    channel: channel,
                    memo: memo,
                    deviceId: deviceId,
                    previousNonce: prevNonce,
                    currentNonce: nextNonce,
                    latitude: 0,
                    longitude: 0,
                    locationAccuracyMeters: 0,
                    locationSource: .offline
                )

                let cbor = try MobileCore.buildAndSignTransaction(input, privateKey: privKey)
                let qr = try MobileCore.encodeQrPayload(cbor)
                let view = try MobileCore.decodeTransaction(cbor)

                try database.appendNonce(
                    counter: counter,
                    nonceHex: nextNonce.toHex(),
                    createdAt: Date()
                )
                try database.enqueuePending(
                    PendingEntry(
                        entryHashHex: MobileCore.blake2b256(cbor).toHex(),
                        sequenceNumber: counter,
                        cbor: cbor,
                        createdAt: Date(),
                        lastAttemptAt: nil,
                        attemptCount: 0
                    )
                )
                try database.upsertTransaction(
                    TransactionRecord(
                        id: view.transactionId,
                        amountMicroOwc: amount,
                        direction: .outgoing,
                        counterpartyPublicKeyHex: recipient.toHex(),
                        counterpartyName: nil,
                        currency: "IQD",
                        channel: channel,
                        memo: memo,
                        timestampUtc: view.timestampUtc,
                        syncStatus: .pending,
                        latitude: 0,
                        longitude: 0,
                        cbor: cbor
                    )
                )

                self.qrPayload = qr
                self.transactionId = view.transactionId
            } catch {
                self.error = error.localizedDescription
            }
        }
    }

    func reset() {
        recipientHex = ""
        amountInput = ""
        memo = ""
        qrPayload = nil
        transactionId = nil
        error = nil
    }

    private func stableDeviceId() throws -> String {
        // Derive a stable UUID v5-style from the installation identifier
        // + bundle id so reinstalls reset the device identity.
        let seed = MobileCore.blake2b256(Data("cs.ios.deviceId".utf8))
        return UUID(uuid: uuidFromBytes(seed.prefix(16))).uuidString
    }

    private func uuidFromBytes(_ data: some Collection<UInt8>) -> uuid_t {
        let b = Array(data.prefix(16)) + Array(repeating: UInt8(0), count: max(0, 16 - data.count))
        return (b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7], b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15])
    }
}
