import SwiftUI

struct WalletView: View {
    @EnvironmentObject private var database: Database
    @EnvironmentObject private var preferences: UserPreferences

    var body: some View {
        NavigationStack {
            VStack(alignment: .leading, spacing: 16) {
                balanceCard
                Text("Recent").font(.headline)
                if database.recentTransactions.isEmpty {
                    Text("Nothing yet — open Receive to show your QR or Scan to accept a payment.")
                        .foregroundStyle(.secondary)
                } else {
                    List(database.recentTransactions) { tx in
                        TransactionRow(transaction: tx)
                    }
                    .listStyle(.plain)
                }
            }
            .padding()
            .navigationTitle("Digital Iraqi Dinar")
            .navigationBarTitleDisplayMode(.inline)
        }
    }

    @ViewBuilder private var balanceCard: some View {
        VStack(alignment: .leading, spacing: 8) {
            Text(preferences.displayName.isEmpty ? "Wallet" : preferences.displayName)
                .font(.subheadline)
                .foregroundStyle(.secondary)
            Text(MoneyFormat.format(database.balanceMicroOwc))
                .font(.system(size: 40, weight: .semibold))
            if database.pendingCount > 0 {
                Text("\(database.pendingCount) pending sync").font(.caption)
            }
        }
        .frame(maxWidth: .infinity, alignment: .leading)
        .padding()
        .background(Color.accentColor.opacity(0.15))
        .cornerRadius(16)
    }
}

struct TransactionRow: View {
    let transaction: TransactionRecord

    var body: some View {
        HStack {
            Image(systemName: transaction.direction == .incoming ? "arrow.down.circle" : "arrow.up.circle")
                .foregroundStyle(transaction.direction == .incoming ? .green : .red)
            VStack(alignment: .leading) {
                Text(transaction.counterpartyName ?? String(transaction.counterpartyPublicKeyHex.prefix(12)))
                Text("\(String(describing: transaction.channel)) · \(transaction.syncStatus.rawValue)")
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }
            Spacer()
            Text(
                (transaction.direction == .incoming ? "+" : "-")
                + MoneyFormat.format(transaction.amountMicroOwc, currency: transaction.currency)
            )
            .font(.callout.monospaced())
        }
    }
}
