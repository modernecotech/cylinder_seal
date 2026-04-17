import SwiftUI

/// Root navigation. Routes to Onboarding if the user hasn't enrolled yet,
/// otherwise to the Wallet tab bar.
struct ContentView: View {
    @EnvironmentObject private var userPreferences: UserPreferences

    var body: some View {
        if userPreferences.isOnboarded {
            MainTabs()
        } else {
            OnboardingView()
        }
    }
}

struct MainTabs: View {
    var body: some View {
        TabView {
            WalletView()
                .tabItem { Label("Wallet", systemImage: "wallet.pass") }

            PayView()
                .tabItem { Label("Send", systemImage: "arrow.up.circle") }

            ReceiveView()
                .tabItem { Label("Receive", systemImage: "arrow.down.circle") }

            HistoryView()
                .tabItem { Label("History", systemImage: "clock") }

            ComplianceView()
                .tabItem { Label("Compliance", systemImage: "checkmark.shield") }

            SettingsView()
                .tabItem { Label("Settings", systemImage: "gearshape") }
        }
    }
}
