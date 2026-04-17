import Foundation

enum KycTier: String, Codable {
    case anonymous = "ANONYMOUS"
    case phoneVerified = "PHONE_VERIFIED"
    case fullKyc = "FULL_KYC"
}

enum PaymentChannel: Int, Codable {
    case nfc = 1
    case ble = 2
    case online = 3
}

enum LocationSource: Int, Codable {
    case unspecified = 0
    case gps = 1
    case network = 2
    case lastKnown = 3
    case offline = 4
}

enum SyncStatus: String, Codable {
    case pending = "PENDING"
    case confirmed = "CONFIRMED"
    case conflicted = "CONFLICTED"
    case rejected = "REJECTED"
}

enum Direction: String, Codable {
    case incoming = "INCOMING"
    case outgoing = "OUTGOING"
}

struct Profile: Codable, Equatable {
    var userId: String
    var displayName: String
    var phoneNumber: String?
    var publicKeyHex: String
    var kycTier: KycTier
    var balanceOwc: Int64
    var createdAt: Date
}

/// A transaction as stored locally (post-decode, display-oriented).
struct TransactionRecord: Identifiable, Codable, Equatable {
    var id: String
    var amountMicroOwc: Int64
    var direction: Direction
    var counterpartyPublicKeyHex: String
    var counterpartyName: String?
    var currency: String
    var channel: PaymentChannel
    var memo: String
    var timestampUtc: Int64
    var syncStatus: SyncStatus
    var latitude: Double
    var longitude: Double
    var cbor: Data
}

struct PendingEntry: Identifiable, Codable, Equatable {
    var id: String { entryHashHex }
    var entryHashHex: String
    var sequenceNumber: Int64
    var cbor: Data
    var createdAt: Date
    var lastAttemptAt: Date?
    var attemptCount: Int
}

struct NonceChainRow: Codable, Equatable {
    var counter: Int64
    var nonceHex: String
    var createdAt: Date
}
