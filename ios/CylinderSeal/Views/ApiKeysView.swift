import SwiftUI

struct ApiKeysView: View {
    @EnvironmentObject private var container: AppContainer
    @State private var keys: [BusinessApiClient.ApiKeyListItem] = []
    @State private var newLabel = ""
    @State private var newSecret: String?
    @State private var errorMessage: String?
    @State private var busy = false

    private let api = BusinessApiClient(baseURL: URL(string: "https://sp-baghdad.cbi.iq")!)

    var body: some View {
        NavigationStack {
            List {
                Section {
                    Text("API keys let your server authenticate against the business API. Each key is shown exactly once — copy it before dismissing.")
                        .font(.caption)
                }
                Section("Issue new") {
                    HStack {
                        TextField("Label", text: $newLabel)
                        Button("Issue") { Task { await issue() } }
                            .disabled(busy || newLabel.trimmingCharacters(in: .whitespaces).isEmpty)
                    }
                }
                if let err = errorMessage {
                    Section { Text(err).foregroundStyle(.red) }
                }
                Section("Active keys") {
                    if keys.isEmpty {
                        Text("None yet.").foregroundStyle(.secondary)
                    } else {
                        ForEach(keys) { key in
                            VStack(alignment: .leading, spacing: 4) {
                                Text(key.label)
                                Text("\(key.key_prefix)… created \(key.created_at)").font(.caption).foregroundStyle(.secondary)
                                if key.revoked {
                                    Text("Revoked").font(.caption).foregroundStyle(.red)
                                } else if let last = key.last_used_at {
                                    Text("Last used: \(last)").font(.caption2).foregroundStyle(.secondary)
                                }
                                if !key.revoked {
                                    Button("Revoke", role: .destructive) { Task { await revoke(key.id) } }
                                        .buttonStyle(.bordered)
                                }
                            }
                            .padding(.vertical, 4)
                        }
                    }
                }
            }
            .navigationTitle("API keys")
            .task { await load() }
            .alert("New API key", isPresented: Binding(
                get: { newSecret != nil },
                set: { if !$0 { newSecret = nil } }
            )) {
                Button("I've saved it") { newSecret = nil }
            } message: {
                if let secret = newSecret {
                    Text("This is the only time the secret will be shown. Copy it now.\n\n\(secret)")
                }
            }
        }
    }

    private func load() async {
        do {
            let pk = try container.walletKeyManager.loadPublicKey()
            let uid = try MobileCore.userIdFromPublicKey(pk)
            keys = try await api.listKeys(userId: uid)
        } catch {
            errorMessage = error.localizedDescription
        }
    }

    private func issue() async {
        busy = true
        defer { busy = false }
        do {
            let pk = try container.walletKeyManager.loadPublicKey()
            let uid = try MobileCore.userIdFromPublicKey(pk)
            let resp = try await api.issueKey(userId: uid, label: newLabel)
            newLabel = ""
            newSecret = resp.secret
            await load()
        } catch {
            errorMessage = error.localizedDescription
        }
    }

    private func revoke(_ id: Int64) async {
        do {
            let pk = try container.walletKeyManager.loadPublicKey()
            let uid = try MobileCore.userIdFromPublicKey(pk)
            try await api.revokeKey(userId: uid, keyId: id)
            await load()
        } catch {
            errorMessage = error.localizedDescription
        }
    }
}
