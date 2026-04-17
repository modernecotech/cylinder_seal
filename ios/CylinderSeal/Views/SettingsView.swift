import SwiftUI

struct SettingsView: View {
    @EnvironmentObject private var preferences: UserPreferences
    @StateObject private var vm: SettingsViewModel

    init() {
        _vm = StateObject(wrappedValue: SettingsViewModel(preferences: UserPreferences()))
    }

    var body: some View {
        NavigationStack {
            Form {
                Section("Super-peer endpoint") {
                    TextField("Host", text: $vm.host)
                        .textInputAutocapitalization(.never)
                        .disableAutocorrection(true)
                    TextField("Port", text: $vm.port)
                        .keyboardType(.numberPad)
                }
                Section {
                    Button("Save", action: vm.save)
                }
                if let last = preferences.lastSyncAt {
                    Section("Sync status") {
                        Text("Last sync: \(last.formatted())")
                    }
                }
                Section("About") {
                    Text("Digital Iraqi Dinar")
                    Text("v0.1.0").font(.caption)
                }
            }
            .navigationTitle("Settings")
        }
    }
}
