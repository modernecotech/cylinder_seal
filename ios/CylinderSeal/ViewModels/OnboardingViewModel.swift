import Foundation

@MainActor
final class OnboardingViewModel: ObservableObject {
    enum Step { case welcome, profile, setPin, generating, done }

    @Published var step: Step = .welcome
    @Published var displayName: String = ""
    @Published var phoneNumber: String = ""
    @Published var pin: String = ""
    @Published var pinConfirm: String = ""
    @Published var errorMessage: String?
    @Published var busy = false
    @Published var publicKeyHex: String?

    private let wallet: WalletKeyManager
    private let preferences: UserPreferences
    private let keychain: KeychainManager

    init(wallet: WalletKeyManager, preferences: UserPreferences, keychain: KeychainManager) {
        self.wallet = wallet
        self.preferences = preferences
        self.keychain = keychain
    }

    func next() {
        errorMessage = nil
        switch step {
        case .welcome:
            step = .profile
        case .profile:
            if displayName.trimmingCharacters(in: .whitespaces).isEmpty {
                errorMessage = "Name required"
                return
            }
            step = .setPin
        case .setPin:
            guard pin.count >= 4 else { errorMessage = "PIN must be 4-6 digits"; return }
            guard pin == pinConfirm else { errorMessage = "PINs don't match"; return }
            step = .generating
            Task { await generate() }
        case .generating, .done:
            break
        }
    }

    private func generate() async {
        busy = true
        defer { busy = false }
        do {
            _ = try keychain.ensureEnclaveKey()
            let pk = try wallet.generateAndStore()
            preferences.completeOnboarding(
                displayName: displayName,
                phoneNumber: phoneNumber.isEmpty ? nil : phoneNumber
            )
            publicKeyHex = pk.toHex()
            step = .done
        } catch {
            errorMessage = "Key generation failed: \(error.localizedDescription)"
            step = .setPin
        }
    }
}
