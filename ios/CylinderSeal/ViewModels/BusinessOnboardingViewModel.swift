import Foundation

enum BusinessKind: String, CaseIterable, Identifiable {
    case pos = "business_pos"
    case electronic = "business_electronic"

    var id: String { rawValue }

    var label: String {
        switch self {
        case .pos: return "Physical point of sale"
        case .electronic: return "Electronic / online commerce"
        }
    }

    var subtitle: String {
        switch self {
        case .pos: return "Shops, market stalls, services with a physical till."
        case .electronic: return "E-commerce, B2B, SaaS. Includes API key access."
        }
    }
}

@MainActor
final class BusinessOnboardingViewModel: ObservableObject {
    @Published var kind: BusinessKind = .pos
    @Published var legalName = ""
    @Published var commercialRegistrationId = ""
    @Published var taxId = ""
    @Published var industryCode = ""
    @Published var registeredAddress = ""
    @Published var contactEmail = ""
    @Published var submitted = false
    @Published var statusMessage: String?
    @Published var errorMessage: String?
    @Published var busy = false

    private let wallet: WalletKeyManager
    private let api: BusinessApiClient

    init(wallet: WalletKeyManager, api: BusinessApiClient) {
        self.wallet = wallet
        self.api = api
    }

    func submit() {
        if [legalName, commercialRegistrationId, taxId, industryCode, registeredAddress, contactEmail]
            .contains(where: { $0.trimmingCharacters(in: .whitespaces).isEmpty })
        {
            errorMessage = "All fields are required"
            return
        }

        Task {
            busy = true
            errorMessage = nil
            defer { busy = false }
            do {
                let publicKey = try wallet.loadPublicKey()
                let userId = try MobileCore.userIdFromPublicKey(publicKey)
                let req = BusinessApiClient.RegisterRequest(
                    user_id: userId,
                    account_type: kind.rawValue,
                    legal_name: legalName,
                    commercial_registration_id: commercialRegistrationId,
                    tax_id: taxId,
                    industry_code: industryCode,
                    registered_address: registeredAddress,
                    contact_email: contactEmail,
                    authorized_signer_public_keys_hex: [publicKey.toHex()]
                )
                let resp = try await api.register(req)
                submitted = true
                statusMessage = "Registered: \(resp.status). CBI ops will verify your commercial registration before activation."
            } catch {
                errorMessage = error.localizedDescription
            }
        }
    }
}
