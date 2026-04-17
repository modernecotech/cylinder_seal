import CryptoKit
import Foundation

/// Ed25519 wallet keypair manager.
///
/// Flow:
/// 1. `generateAndStore()` produces a fresh keypair via the Rust core,
///    then wraps the private key under an AES-GCM key derived from the
///    Secure Enclave (see `KeychainManager.deriveWrapKey`).
/// 2. `loadPrivateKey()` unwraps it when a payment needs signing. The
///    returned buffer should be zeroed by the caller after use.
@MainActor
final class WalletKeyManager: ObservableObject {
    @Published private(set) var publicKey: Data?

    private let keychain: KeychainManager
    private let publicKeyURL: URL
    private let aad = Data("cs.wallet.v1".utf8)

    init(keychain: KeychainManager) {
        self.keychain = keychain
        let fm = FileManager.default
        let docs = try? fm.url(for: .applicationSupportDirectory, in: .userDomainMask, appropriateFor: nil, create: true)
        self.publicKeyURL = (docs ?? fm.temporaryDirectory).appendingPathComponent("wallet_public.bin")
        self.publicKey = try? Data(contentsOf: publicKeyURL)
    }

    var hasWallet: Bool { publicKey != nil }

    /// Generate a new Ed25519 keypair, persist it wrapped under the
    /// Secure Enclave-derived key, and return the public half.
    @discardableResult
    func generateAndStore() throws -> Data {
        let kp = try MobileCore.generateKeypair()

        let wrapKey = try keychain.deriveWrapKey()
        let sealed = try AES.GCM.seal(kp.privateKey, using: wrapKey, authenticating: aad)
        let combined = sealed.combined ?? (sealed.nonce.withUnsafeBytes { Data($0) } + sealed.ciphertext + sealed.tag)
        try keychain.storeWrappedWallet(combined)
        try kp.publicKey.write(to: publicKeyURL, options: [.atomic, .completeFileProtection])
        self.publicKey = kp.publicKey
        return kp.publicKey
    }

    /// Unwrap and return the private key. Caller should zero it after use.
    func loadPrivateKey() throws -> Data {
        guard let wrapped = try keychain.loadWrappedWallet() else {
            throw WalletKeyError.noWallet
        }
        let wrapKey = try keychain.deriveWrapKey()
        let sealed = try AES.GCM.SealedBox(combined: wrapped)
        return try AES.GCM.open(sealed, using: wrapKey, authenticating: aad)
    }

    func loadPublicKey() throws -> Data {
        if let pk = publicKey { return pk }
        let pk = try Data(contentsOf: publicKeyURL)
        self.publicKey = pk
        return pk
    }

    func reset() throws {
        try keychain.deleteWrappedWallet()
        try? FileManager.default.removeItem(at: publicKeyURL)
        self.publicKey = nil
    }
}

enum WalletKeyError: Error, LocalizedError {
    case noWallet

    var errorDescription: String? {
        switch self {
        case .noWallet: return "No wallet has been generated yet."
        }
    }
}
