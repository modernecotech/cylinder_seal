import Foundation

/// REST client for the super-peer's `/v1/businesses` surface.
/// All requests run over URLSession with JSONEncoder/Decoder; no extra
/// Retrofit-equivalent dependency needed on iOS.
@MainActor
final class BusinessApiClient: ObservableObject {
    private let baseURL: URL
    private let session: URLSession
    private let encoder: JSONEncoder
    private let decoder: JSONDecoder

    init(baseURL: URL, session: URLSession = .shared) {
        self.baseURL = baseURL
        self.session = session
        self.encoder = JSONEncoder()
        self.decoder = JSONDecoder()
    }

    // MARK: - Registration

    struct RegisterRequest: Encodable {
        let user_id: String
        let account_type: String
        let legal_name: String
        let commercial_registration_id: String
        let tax_id: String
        let industry_code: String
        let registered_address: String
        let contact_email: String
        let authorized_signer_public_keys_hex: [String]
    }

    struct RegisterResponse: Decodable {
        let status: String
        let user_id: String
    }

    func register(_ req: RegisterRequest) async throws -> RegisterResponse {
        try await post("v1/businesses", body: req)
    }

    // MARK: - Profile

    struct BusinessProfileDto: Decodable {
        let user_id: String
        let legal_name: String
        let commercial_registration_id: String
        let tax_id: String
        let industry_code: String
        let registered_address: String
        let contact_email: String
        let signature_threshold: Int
        let daily_volume_limit_owc: Int64
        let edd_cleared: Bool
        let approved_at: String?
    }

    func getProfile(userId: String) async throws -> BusinessProfileDto {
        try await get("v1/businesses/\(userId)")
    }

    // MARK: - API keys

    struct IssueKeyRequest: Encodable {
        let label: String
        let scopes: [String]
    }

    struct IssueKeyResponse: Decodable {
        let id: Int64
        let key_prefix: String
        let secret: String
        let label: String
    }

    struct ApiKeyListItem: Decodable, Identifiable {
        var id: Int64 { self.id_value }
        let id_value: Int64
        let key_prefix: String
        let label: String
        let created_at: String
        let last_used_at: String?
        let revoked: Bool

        private enum CodingKeys: String, CodingKey {
            case id_value = "id"
            case key_prefix, label, created_at, last_used_at, revoked
        }
    }

    func issueKey(userId: String, label: String, scopes: [String] = []) async throws -> IssueKeyResponse {
        try await post("v1/businesses/\(userId)/api-keys", body: IssueKeyRequest(label: label, scopes: scopes))
    }

    func listKeys(userId: String) async throws -> [ApiKeyListItem] {
        try await get("v1/businesses/\(userId)/api-keys")
    }

    func revokeKey(userId: String, keyId: Int64) async throws {
        let url = baseURL.appendingPathComponent("v1/businesses/\(userId)/api-keys/\(keyId)")
        var req = URLRequest(url: url)
        req.httpMethod = "DELETE"
        let (_, resp) = try await session.data(for: req)
        try ensureSuccess(resp)
    }

    // MARK: - Internals

    private func get<T: Decodable>(_ path: String) async throws -> T {
        let url = baseURL.appendingPathComponent(path)
        var req = URLRequest(url: url)
        req.httpMethod = "GET"
        let (data, resp) = try await session.data(for: req)
        try ensureSuccess(resp)
        return try decoder.decode(T.self, from: data)
    }

    private func post<Req: Encodable, Resp: Decodable>(_ path: String, body: Req) async throws -> Resp {
        let url = baseURL.appendingPathComponent(path)
        var req = URLRequest(url: url)
        req.httpMethod = "POST"
        req.setValue("application/json", forHTTPHeaderField: "Content-Type")
        req.httpBody = try encoder.encode(body)
        let (data, resp) = try await session.data(for: req)
        try ensureSuccess(resp)
        return try decoder.decode(Resp.self, from: data)
    }

    private func ensureSuccess(_ resp: URLResponse) throws {
        guard let http = resp as? HTTPURLResponse, (200..<300).contains(http.statusCode) else {
            let code = (resp as? HTTPURLResponse)?.statusCode ?? -1
            throw BusinessApiError.http(code)
        }
    }
}

enum BusinessApiError: Error, LocalizedError {
    case http(Int)

    var errorDescription: String? {
        switch self {
        case .http(let code): return "HTTP \(code)"
        }
    }
}
