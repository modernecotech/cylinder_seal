import SwiftUI

/// "Why was this held?" — surfaces recent compliance evaluations from
/// `/v1/compliance/users/:userId/explanations`.
struct ComplianceView: View {
    @EnvironmentObject private var container: AppContainer
    @State private var items: [ComplianceApiClient.RecentEvalDto] = []
    @State private var errorMessage: String?
    @State private var loading = false

    private let api = ComplianceApiClient(baseURL: URL(string: "https://sp-baghdad.cbi.iq")!)

    var body: some View {
        NavigationStack {
            List {
                Section {
                    Text("Recent transactions reviewed by our compliance system. If something was held, the explanation tells you why.")
                        .font(.caption)
                }
                if loading && items.isEmpty {
                    Text("Loading…").foregroundStyle(.secondary)
                }
                if let err = errorMessage {
                    Section { Text(err).foregroundStyle(.red) }
                }
                Section("Recent reviews") {
                    if items.isEmpty && !loading {
                        Text("Nothing on file yet.").foregroundStyle(.secondary)
                    }
                    ForEach(items) { row in
                        EvalRow(row: row)
                    }
                }
            }
            .navigationTitle("Compliance")
            .task { await load() }
            .refreshable { await load() }
        }
    }

    private func load() async {
        loading = true
        defer { loading = false }
        do {
            let pk = try container.walletKeyManager.loadPublicKey()
            let uid = try MobileCore.userIdFromPublicKey(pk)
            let resp = try await api.explanations(userId: uid)
            items = resp.recent
            errorMessage = nil
        } catch {
            errorMessage = error.localizedDescription
        }
    }
}

private struct EvalRow: View {
    let row: ComplianceApiClient.RecentEvalDto

    var body: some View {
        VStack(alignment: .leading, spacing: 4) {
            HStack {
                Text("\(row.risk_level) (score \(row.composite_score))")
                    .font(.callout.bold())
                    .foregroundStyle(toneColor(row.risk_level))
                if row.held_for_review {
                    Spacer()
                    Text("HELD")
                        .font(.caption2)
                        .padding(.horizontal, 6)
                        .padding(.vertical, 2)
                        .background(Color.orange.opacity(0.2))
                        .foregroundStyle(Color.orange)
                        .cornerRadius(4)
                }
            }
            Text("Action: \(row.recommended_action)")
                .font(.caption)
                .foregroundStyle(.secondary)
            Text(row.explanation)
                .font(.footnote)
            Text(row.evaluated_at)
                .font(.caption2)
                .foregroundStyle(.secondary)
        }
        .padding(.vertical, 2)
    }

    private func toneColor(_ level: String) -> Color {
        switch level {
        case "Critical": return .red
        case "High": return .orange
        case "Medium": return .yellow
        case "MediumLow": return .mint
        default: return .green
        }
    }
}
