import Foundation
import GRPC
import NIOCore
import NIOPosix
import SwiftProtobuf

/// gRPC client for the super-peer `ChainSync` service.
///
/// On iOS we send one pending entry per `SyncChain` call (rather than
/// fully interactive bidirectional streaming) because BGTaskScheduler
/// execution windows are short (~30s). Server-streaming inside a single
/// RPC is fine; we just don't hold the stream open indefinitely.
@MainActor
final class ChainSyncClient: ObservableObject {
    private let userPreferences: UserPreferences
    private let database: Database
    private var group: MultiThreadedEventLoopGroup?
    private var channel: ClientConnection?
    private var client: Iq_Cbi_Cylinderseal_Chainsync_ChainSyncAsyncClient?

    init(userPreferences: UserPreferences, database: Database) {
        self.userPreferences = userPreferences
        self.database = database
    }

    /// Drain the pending queue once. Safe to call from `SyncWorker`.
    func drainOnce() async {
        await ensureChannel()
        guard let client else { return }

        let pending: [PendingEntry]
        do { pending = try database.drainPending() } catch {
            NSLog("drain failed: \(error)")
            return
        }
        if pending.isEmpty { return }

        do {
            let call = client.makeSyncChainCall()
            let writeTask = Task {
                for p in pending {
                    var entry = Iq_Cbi_Cylinderseal_Chainsync_JournalEntry()
                    entry.entryHash = Data(hex: p.entryHashHex) ?? Data()
                    entry.sequenceNumber = UInt64(p.sequenceNumber)
                    entry.createdAt = Int64(p.createdAt.timeIntervalSince1970 * 1_000_000)
                    try await call.requestStream.send(entry)
                }
                try await call.requestStream.finish()
            }

            for try await ack in call.responseStream {
                let entryHashHex = ack.entryID.toHex()
                switch ack.status {
                case .confirmed, .rejected:
                    try database.removePending(entryHashHex: entryHashHex)
                    try database.updateStatus(
                        id: try MobileCore.userIdFromPublicKey(Data()),
                        status: ack.status == .confirmed ? .confirmed : .rejected
                    )
                case .conflicted, .pending:
                    try database.recordPendingAttempt(entryHashHex: entryHashHex, now: Date())
                default:
                    break
                }
            }
            _ = try await writeTask.value
            userPreferences.recordSync(at: Date())
        } catch {
            NSLog("sync drain failed: \(error)")
            for p in pending {
                try? database.recordPendingAttempt(entryHashHex: p.entryHashHex, now: Date())
            }
        }
    }

    private func ensureChannel() async {
        if client != nil { return }
        let group = self.group ?? MultiThreadedEventLoopGroup(numberOfThreads: 1)
        self.group = group

        let builder = ClientConnection.usingPlatformAppropriateTLS(for: group)
        let channel = builder.connect(
            host: userPreferences.superpeerHost,
            port: userPreferences.superpeerPort
        )
        self.channel = channel
        self.client = Iq_Cbi_Cylinderseal_Chainsync_ChainSyncAsyncClient(channel: channel)
    }

    func shutdown() {
        do {
            try channel?.close().wait()
            try group?.syncShutdownGracefully()
        } catch {
            NSLog("gRPC shutdown error: \(error)")
        }
        channel = nil
        group = nil
        client = nil
    }
}
