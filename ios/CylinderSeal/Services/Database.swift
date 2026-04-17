import Combine
import Foundation
import SQLite

/// SQLite-backed local store. Uses `NSFileProtectionComplete` on the file
/// so data is encrypted by iOS when the device is locked — equivalent to
/// SQLCipher for practical device-seizure attacks, without the third-party
/// dependency.
@MainActor
final class Database: ObservableObject {
    private let connection: Connection
    private let url: URL

    /// Mutable caches exposed to SwiftUI.
    @Published private(set) var recentTransactions: [TransactionRecord] = []
    @Published private(set) var pendingCount: Int = 0
    @Published private(set) var balanceMicroOwc: Int64 = 0

    // MARK: - Schema

    private let transactionsTable = Table("transactions")
    private let idCol = Expression<String>("transaction_id")
    private let amountCol = Expression<Int64>("amount_micro_owc")
    private let directionCol = Expression<String>("direction")
    private let counterpartyPkCol = Expression<String>("counterparty_pk_hex")
    private let counterpartyNameCol = Expression<String?>("counterparty_name")
    private let currencyCol = Expression<String>("currency")
    private let channelCol = Expression<Int>("channel")
    private let memoCol = Expression<String>("memo")
    private let timestampCol = Expression<Int64>("timestamp_utc")
    private let syncStatusCol = Expression<String>("sync_status")
    private let latCol = Expression<Double>("latitude")
    private let lonCol = Expression<Double>("longitude")
    private let cborCol = Expression<Data>("cbor")

    private let pendingTable = Table("pending_entries")
    private let entryHashCol = Expression<String>("entry_hash_hex")
    private let seqCol = Expression<Int64>("sequence_number")
    private let pendingCborCol = Expression<Data>("cbor")
    private let createdAtCol = Expression<Int64>("created_at_ms")
    private let lastAttemptCol = Expression<Int64?>("last_attempt_at_ms")
    private let attemptCountCol = Expression<Int>("attempt_count")

    private let nonceTable = Table("nonce_chain")
    private let counterCol = Expression<Int64>("counter")
    private let nonceHexCol = Expression<String>("nonce_hex")
    private let nonceCreatedCol = Expression<Int64>("created_at_ms")

    // MARK: - Setup

    private init(connection: Connection, url: URL) throws {
        self.connection = connection
        self.url = url
        try migrate()
        try refresh()
    }

    static func open() throws -> Database {
        let fm = FileManager.default
        let dir = try fm.url(
            for: .applicationSupportDirectory,
            in: .userDomainMask,
            appropriateFor: nil,
            create: true
        )
        let url = dir.appendingPathComponent("cylinder_seal.sqlite")
        let conn = try Connection(url.path)
        try fm.setAttributes(
            [.protectionKey: FileProtectionType.complete],
            ofItemAtPath: url.path
        )
        return try Database(connection: conn, url: url)
    }

    /// Fallback for the unusual case where file-system DB open fails
    /// (disk full, etc.). Lets the UI come up so the user sees a coherent
    /// error state instead of a blank screen.
    static func inMemoryFallback() -> Database {
        let url = URL(fileURLWithPath: "/dev/null")
        do {
            let conn = try Connection(.inMemory)
            return try Database(connection: conn, url: url)
        } catch {
            fatalError("in-memory SQLite failed: \(error)")
        }
    }

    private func migrate() throws {
        try connection.run(transactionsTable.create(ifNotExists: true) { t in
            t.column(idCol, primaryKey: true)
            t.column(amountCol)
            t.column(directionCol)
            t.column(counterpartyPkCol)
            t.column(counterpartyNameCol)
            t.column(currencyCol)
            t.column(channelCol)
            t.column(memoCol)
            t.column(timestampCol)
            t.column(syncStatusCol)
            t.column(latCol)
            t.column(lonCol)
            t.column(cborCol)
        })
        try connection.run(transactionsTable.createIndex(timestampCol, ifNotExists: true))
        try connection.run(pendingTable.create(ifNotExists: true) { t in
            t.column(entryHashCol, primaryKey: true)
            t.column(seqCol, unique: true)
            t.column(pendingCborCol)
            t.column(createdAtCol)
            t.column(lastAttemptCol)
            t.column(attemptCountCol, defaultValue: 0)
        })
        try connection.run(nonceTable.create(ifNotExists: true) { t in
            t.column(counterCol, primaryKey: true)
            t.column(nonceHexCol)
            t.column(nonceCreatedCol)
        })
    }

    // MARK: - Transactions

    func upsertTransaction(_ tx: TransactionRecord) throws {
        let stmt = transactionsTable.insert(or: .replace,
            idCol <- tx.id,
            amountCol <- tx.amountMicroOwc,
            directionCol <- tx.direction.rawValue,
            counterpartyPkCol <- tx.counterpartyPublicKeyHex,
            counterpartyNameCol <- tx.counterpartyName,
            currencyCol <- tx.currency,
            channelCol <- tx.channel.rawValue,
            memoCol <- tx.memo,
            timestampCol <- tx.timestampUtc,
            syncStatusCol <- tx.syncStatus.rawValue,
            latCol <- tx.latitude,
            lonCol <- tx.longitude,
            cborCol <- tx.cbor
        )
        try connection.run(stmt)
        try refresh()
    }

    func updateStatus(id: String, status: SyncStatus) throws {
        try connection.run(transactionsTable.filter(idCol == id).update(syncStatusCol <- status.rawValue))
        try refresh()
    }

    func recent(limit: Int = 50) throws -> [TransactionRecord] {
        try connection.prepare(transactionsTable.order(timestampCol.desc).limit(limit))
            .map(row: self)
    }

    // MARK: - Pending queue

    func enqueuePending(_ entry: PendingEntry) throws {
        let stmt = pendingTable.insert(or: .ignore,
            entryHashCol <- entry.entryHashHex,
            seqCol <- entry.sequenceNumber,
            pendingCborCol <- entry.cbor,
            createdAtCol <- Int64(entry.createdAt.timeIntervalSince1970 * 1000),
            lastAttemptCol <- entry.lastAttemptAt.map { Int64($0.timeIntervalSince1970 * 1000) },
            attemptCountCol <- entry.attemptCount
        )
        try connection.run(stmt)
        try refresh()
    }

    func drainPending() throws -> [PendingEntry] {
        try connection.prepare(pendingTable.order(seqCol.asc)).map { row in
            PendingEntry(
                entryHashHex: row[entryHashCol],
                sequenceNumber: row[seqCol],
                cbor: row[pendingCborCol],
                createdAt: Date(timeIntervalSince1970: TimeInterval(row[createdAtCol]) / 1000),
                lastAttemptAt: row[lastAttemptCol].map { Date(timeIntervalSince1970: TimeInterval($0) / 1000) },
                attemptCount: row[attemptCountCol]
            )
        }
    }

    func removePending(entryHashHex: String) throws {
        try connection.run(pendingTable.filter(entryHashCol == entryHashHex).delete())
        try refresh()
    }

    func recordPendingAttempt(entryHashHex: String, now: Date) throws {
        try connection.run(pendingTable.filter(entryHashCol == entryHashHex).update(
            attemptCountCol += 1,
            lastAttemptCol <- Int64(now.timeIntervalSince1970 * 1000)
        ))
    }

    // MARK: - Nonce chain

    func latestNonce() throws -> NonceChainRow? {
        guard let row = try connection.pluck(nonceTable.order(counterCol.desc).limit(1)) else { return nil }
        return NonceChainRow(
            counter: row[counterCol],
            nonceHex: row[nonceHexCol],
            createdAt: Date(timeIntervalSince1970: TimeInterval(row[nonceCreatedCol]) / 1000)
        )
    }

    func appendNonce(counter: Int64, nonceHex: String, createdAt: Date) throws {
        try connection.run(nonceTable.insert(or: .replace,
            counterCol <- counter,
            nonceHexCol <- nonceHex,
            nonceCreatedCol <- Int64(createdAt.timeIntervalSince1970 * 1000)
        ))
    }

    // MARK: - Computed cache

    private func refresh() throws {
        self.recentTransactions = try recent()
        self.pendingCount = try connection.scalar(pendingTable.count)
        self.balanceMicroOwc = try computeBalance()
    }

    private func computeBalance() throws -> Int64 {
        let incoming = try connection.scalar(
            transactionsTable
                .filter(directionCol == "INCOMING" && (syncStatusCol == "CONFIRMED" || syncStatusCol == "PENDING"))
                .select(amountCol.sum)
        ) ?? 0
        let outgoing = try connection.scalar(
            transactionsTable
                .filter(directionCol == "OUTGOING" && (syncStatusCol == "CONFIRMED" || syncStatusCol == "PENDING"))
                .select(amountCol.sum)
        ) ?? 0
        return incoming - outgoing
    }
}

private extension AnySequence where Element == SQLite.Row {
    func map(row host: Database) -> [TransactionRecord] {
        compactMap { row in
            guard
                let dir = Direction(rawValue: row[Expression<String>("direction")]),
                let status = SyncStatus(rawValue: row[Expression<String>("sync_status")]),
                let channel = PaymentChannel(rawValue: row[Expression<Int>("channel")])
            else { return nil }
            return TransactionRecord(
                id: row[Expression<String>("transaction_id")],
                amountMicroOwc: row[Expression<Int64>("amount_micro_owc")],
                direction: dir,
                counterpartyPublicKeyHex: row[Expression<String>("counterparty_pk_hex")],
                counterpartyName: row[Expression<String?>("counterparty_name")],
                currency: row[Expression<String>("currency")],
                channel: channel,
                memo: row[Expression<String>("memo")],
                timestampUtc: row[Expression<Int64>("timestamp_utc")],
                syncStatus: status,
                latitude: row[Expression<Double>("latitude")],
                longitude: row[Expression<Double>("longitude")],
                cbor: row[Expression<Data>("cbor")]
            )
        }
    }
}
