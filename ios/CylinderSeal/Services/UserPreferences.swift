import Combine
import Foundation

/// Lightweight preferences store. Uses `UserDefaults` because the only
/// sensitive material (the wallet private key) lives in the Keychain.
@MainActor
final class UserPreferences: ObservableObject {
    @Published private(set) var isOnboarded: Bool
    @Published private(set) var displayName: String
    @Published private(set) var phoneNumber: String?
    @Published private(set) var kycTier: KycTier
    @Published private(set) var superpeerHost: String
    @Published private(set) var superpeerPort: Int
    @Published private(set) var lastSyncAt: Date?

    private let defaults: UserDefaults

    init(defaults: UserDefaults = .standard) {
        self.defaults = defaults
        self.isOnboarded = defaults.bool(forKey: Keys.onboarded)
        self.displayName = defaults.string(forKey: Keys.displayName) ?? ""
        self.phoneNumber = defaults.string(forKey: Keys.phoneNumber)
        self.kycTier = KycTier(rawValue: defaults.string(forKey: Keys.kycTier) ?? "") ?? .anonymous
        self.superpeerHost = defaults.string(forKey: Keys.superpeerHost) ?? "sp-baghdad.cbi.iq"
        self.superpeerPort = defaults.integer(forKey: Keys.superpeerPort).nonZeroOr(50_051)
        self.lastSyncAt = defaults.object(forKey: Keys.lastSyncAt) as? Date
    }

    func completeOnboarding(displayName: String, phoneNumber: String?) {
        self.isOnboarded = true
        self.displayName = displayName
        self.phoneNumber = phoneNumber
        defaults.set(true, forKey: Keys.onboarded)
        defaults.set(displayName, forKey: Keys.displayName)
        if let phoneNumber { defaults.set(phoneNumber, forKey: Keys.phoneNumber) }
    }

    func setKycTier(_ tier: KycTier) {
        self.kycTier = tier
        defaults.set(tier.rawValue, forKey: Keys.kycTier)
    }

    func setSuperpeer(host: String, port: Int) {
        self.superpeerHost = host
        self.superpeerPort = port
        defaults.set(host, forKey: Keys.superpeerHost)
        defaults.set(port, forKey: Keys.superpeerPort)
    }

    func recordSync(at date: Date) {
        self.lastSyncAt = date
        defaults.set(date, forKey: Keys.lastSyncAt)
    }

    func reset() {
        for k in Keys.all { defaults.removeObject(forKey: k) }
        self.isOnboarded = false
        self.displayName = ""
        self.phoneNumber = nil
        self.kycTier = .anonymous
        self.lastSyncAt = nil
    }

    private enum Keys {
        static let onboarded = "cs.onboarded"
        static let displayName = "cs.displayName"
        static let phoneNumber = "cs.phoneNumber"
        static let kycTier = "cs.kycTier"
        static let superpeerHost = "cs.superpeerHost"
        static let superpeerPort = "cs.superpeerPort"
        static let lastSyncAt = "cs.lastSyncAt"

        static let all: [String] = [onboarded, displayName, phoneNumber, kycTier, superpeerHost, superpeerPort, lastSyncAt]
    }
}

private extension Int {
    func nonZeroOr(_ fallback: Int) -> Int {
        self == 0 ? fallback : self
    }
}
