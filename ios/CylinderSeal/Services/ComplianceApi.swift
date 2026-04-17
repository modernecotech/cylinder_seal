import Foundation

/// REST client for /v1/compliance — read-only "why was this held?" view
/// for end users. Same shape as the Android `ComplianceApi`; serde
/// snake_case names preserved.
@MainActor
final class ComplianceApiClient: ObservableObject {
    private let baseURL: URL
    private let session: URLSession
    private let decoder: JSONDecoder

    init(baseURL: URL, session: URLSession = .shared) {
        self.baseURL = baseURL
        self.session = session
        self.decoder = JSONDecoder()
    }

    struct RecentEvalDto: Decodable, Identifiable {
        var id: String { transaction_id }
        let transaction_id: String
        let composite_score: Int
        let risk_level: String
        let held_for_review: Bool
        let recommended_action: String
        let explanation: String
        let evaluated_at: String
    }

    struct UserExplanationResponse: Decodable {
        let user_id: String
        let recent: [RecentEvalDto]
    }

    func explanations(userId: String, limit: Int = 20) async throws -> UserExplanationResponse {
        var comps = URLComponents(
            url: baseURL.appendingPathComponent("v1/compliance/users/\(userId)/explanations"),
            resolvingAgainstBaseURL: false
        )!
        comps.queryItems = [URLQueryItem(name: "limit", value: String(limit))]
        var req = URLRequest(url: comps.url!)
        req.httpMethod = "GET"
        let (data, resp) = try await session.data(for: req)
        guard let http = resp as? HTTPURLResponse, (200..<300).contains(http.statusCode) else {
            throw ComplianceApiError.http((resp as? HTTPURLResponse)?.statusCode ?? -1)
        }
        return try decoder.decode(UserExplanationResponse.self, from: data)
    }
}

enum ComplianceApiError: Error, LocalizedError {
    case http(Int)
    var errorDescription: String? {
        switch self {
        case .http(let code): return "HTTP \(code)"
        }
    }
}
