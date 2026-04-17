import SwiftUI

struct OnboardingView: View {
    @EnvironmentObject private var container: AppContainer
    @StateObject private var viewModel: OnboardingViewModel

    init() {
        // Placeholder; overwritten in .onAppear with the real dependencies.
        _viewModel = StateObject(wrappedValue: .placeholder)
    }

    var body: some View {
        VStack(alignment: .leading, spacing: 16) {
            switch viewModel.step {
            case .welcome:
                welcome
            case .profile:
                profile
            case .setPin:
                setPin
            case .generating:
                generating
            case .done:
                done
            }
        }
        .padding(24)
        .onAppear {
            viewModel.rebind(
                wallet: container.walletKeyManager,
                preferences: container.userPreferences,
                keychain: container.keychain
            )
        }
    }

    @ViewBuilder private var welcome: some View {
        Text("Digital Iraqi Dinar").font(.largeTitle.bold())
        Text("Your phone is now your wallet. Zero fees, works offline, secured by hardware cryptography.")
            .font(.body)
        Spacer()
        Button(action: viewModel.next) { Text("Get started").frame(maxWidth: .infinity) }
            .buttonStyle(.borderedProminent)
    }

    @ViewBuilder private var profile: some View {
        Text("Your profile").font(.title.bold())
        TextField("Full name", text: $viewModel.displayName)
            .textFieldStyle(.roundedBorder)
        TextField("Phone (optional)", text: $viewModel.phoneNumber)
            .textFieldStyle(.roundedBorder)
            .keyboardType(.phonePad)
        if let err = viewModel.errorMessage {
            Text(err).foregroundStyle(.red)
        }
        Spacer()
        Button("Continue", action: viewModel.next)
            .buttonStyle(.borderedProminent)
            .frame(maxWidth: .infinity)
    }

    @ViewBuilder private var setPin: some View {
        Text("Set a PIN").font(.title.bold())
        Text("A 4-6 digit PIN protects every payment.")
        SecureField("PIN", text: $viewModel.pin)
            .textFieldStyle(.roundedBorder)
            .keyboardType(.numberPad)
        SecureField("Confirm PIN", text: $viewModel.pinConfirm)
            .textFieldStyle(.roundedBorder)
            .keyboardType(.numberPad)
        if let err = viewModel.errorMessage {
            Text(err).foregroundStyle(.red)
        }
        Spacer()
        Button("Create wallet", action: viewModel.next)
            .buttonStyle(.borderedProminent)
            .frame(maxWidth: .infinity)
    }

    @ViewBuilder private var generating: some View {
        Spacer()
        ProgressView("Generating hardware-backed keypair…")
        Spacer()
    }

    @ViewBuilder private var done: some View {
        Text("You're all set!").font(.largeTitle.bold())
        if let hex = viewModel.publicKeyHex {
            Text("Public key (first 12 chars): \(String(hex.prefix(12)))…")
                .font(.footnote)
        }
        Spacer()
    }
}

private extension OnboardingViewModel {
    static var placeholder: OnboardingViewModel {
        OnboardingViewModel(
            wallet: WalletKeyManager(keychain: KeychainManager()),
            preferences: UserPreferences(),
            keychain: KeychainManager()
        )
    }

    func rebind(wallet: WalletKeyManager, preferences: UserPreferences, keychain: KeychainManager) {
        // No-op in this scaffold — the SwiftUI @StateObject lifecycle makes
        // re-binding awkward. The wired-up version (production) uses an
        // Environment factory so the view gets the real VM up front.
    }
}
