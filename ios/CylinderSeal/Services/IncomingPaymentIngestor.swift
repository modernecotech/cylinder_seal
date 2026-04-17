import Foundation

/// Single chokepoint for every inbound signed-CBOR payload, regardless of
/// transport (NFC, BLE, QR).
///
/// Validates the signature via the Rust core, then:
///   1. inserts a pending-queue row so `SyncWorker` will ship it
///      up to the super-peer,
///   2. inserts a `transactions` row so the UI sees the money immediately.
///
/// Returns `false` on reject so the caller (the NFC/BLE transport) can
/// signal the sender to retry.
@MainActor
final class IncomingPaymentIngestor: ObservableObject {
    @Published private(set) var lastIngestStatus: Status = .idle

    enum Status: Equatable {
        case idle
        case accepted(transactionId: String)
        case rejected(reason: String)
    }

    private let database: Database

    init(database: Database) {
        self.database = database
    }

    @discardableResult
    func onPayload(_ cbor: Data, transport: PaymentChannel) -> Bool {
        do {
            let view = try MobileCore.decodeTransaction(cbor)
            guard view.signatureValid else {
                lastIngestStatus = .rejected(reason: "Signature failed verification")
                return false
            }
            let entryHash = MobileCore.blake2b256(cbor)

            let nextSeq: Int64 = {
                (try? database.drainPending().last?.sequenceNumber ?? 0) ?? 0
            }() + 1

            try database.enqueuePending(
                PendingEntry(
                    entryHashHex: entryHash.toHex(),
                    sequenceNumber: nextSeq,
                    cbor: cbor,
                    createdAt: Date(),
                    lastAttemptAt: nil,
                    attemptCount: 0
                )
            )

            let record = TransactionRecord(
                id: view.transactionId,
                amountMicroOwc: view.amountMicroOwc,
                direction: .incoming,
                counterpartyPublicKeyHex: view.fromPublicKey.toHex(),
                counterpartyName: nil,
                currency: view.currencyContext,
                channel: transport,
                memo: view.memo,
                timestampUtc: view.timestampUtc,
                syncStatus: .pending,
                latitude: 0,
                longitude: 0,
                cbor: cbor
            )
            try database.upsertTransaction(record)

            lastIngestStatus = .accepted(transactionId: view.transactionId)
            return true
        } catch {
            lastIngestStatus = .rejected(reason: error.localizedDescription)
            return false
        }
    }
}
