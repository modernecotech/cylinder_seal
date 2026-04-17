import Foundation

@MainActor
final class SettingsViewModel: ObservableObject {
    @Published var host: String
    @Published var port: String

    private let preferences: UserPreferences

    init(preferences: UserPreferences) {
        self.preferences = preferences
        self.host = preferences.superpeerHost
        self.port = String(preferences.superpeerPort)
    }

    func save() {
        guard let portInt = Int(port), portInt > 0 else { return }
        preferences.setSuperpeer(host: host, port: portInt)
    }
}
