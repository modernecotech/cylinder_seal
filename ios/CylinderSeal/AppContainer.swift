import Foundation

/// Dependency container. Holds the singletons the UI and ViewModels need.
///
/// `bootstrap()` does the minimum work required before the first
/// SwiftUI frame: opens the database, loads user preferences, stands up the
/// managers. Anything that can be lazy is.
@MainActor
final class AppContainer: ObservableObject {
    let userPreferences: UserPreferences
    let keychain: KeychainManager
    let walletKeyManager: WalletKeyManager
    let database: Database
    let ingestor: IncomingPaymentIngestor
    let bleService: BLEService
    let nfcReader: NFCReader
    let syncClient: ChainSyncClient

    private init(
        userPreferences: UserPreferences,
        keychain: KeychainManager,
        walletKeyManager: WalletKeyManager,
        database: Database,
        ingestor: IncomingPaymentIngestor,
        bleService: BLEService,
        nfcReader: NFCReader,
        syncClient: ChainSyncClient
    ) {
        self.userPreferences = userPreferences
        self.keychain = keychain
        self.walletKeyManager = walletKeyManager
        self.database = database
        self.ingestor = ingestor
        self.bleService = bleService
        self.nfcReader = nfcReader
        self.syncClient = syncClient
    }

    static func bootstrap() -> AppContainer {
        let userPreferences = UserPreferences()
        let keychain = KeychainManager()
        let walletKeyManager = WalletKeyManager(keychain: keychain)
        let database: Database
        do {
            database = try Database.open()
        } catch {
            // A failure here is unrecoverable — the app can't write. We
            // surface the error on-screen via a fallback in ContentView.
            database = Database.inMemoryFallback()
            NSLog("Database.open failed: \(error)")
        }
        let ingestor = IncomingPaymentIngestor(database: database)
        let bleService = BLEService(ingestor: ingestor)
        let nfcReader = NFCReader(ingestor: ingestor)
        let syncClient = ChainSyncClient(userPreferences: userPreferences, database: database)

        return AppContainer(
            userPreferences: userPreferences,
            keychain: keychain,
            walletKeyManager: walletKeyManager,
            database: database,
            ingestor: ingestor,
            bleService: bleService,
            nfcReader: nfcReader,
            syncClient: syncClient
        )
    }
}
