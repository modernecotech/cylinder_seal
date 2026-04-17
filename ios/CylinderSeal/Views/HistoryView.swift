import SwiftUI

struct HistoryView: View {
    @EnvironmentObject private var database: Database

    var body: some View {
        NavigationStack {
            List(database.recentTransactions) { tx in
                VStack(alignment: .leading, spacing: 4) {
                    HStack {
                        Text(
                            (tx.direction == .incoming ? "+" : "-")
                            + MoneyFormat.format(tx.amountMicroOwc, currency: tx.currency)
                        )
                        .font(.callout.monospaced())
                        Spacer()
                        Text(tx.syncStatus.rawValue)
                            .font(.caption2)
                            .padding(.horizontal, 6)
                            .padding(.vertical, 2)
                            .background(statusColor(tx.syncStatus).opacity(0.2))
                            .foregroundStyle(statusColor(tx.syncStatus))
                            .cornerRadius(4)
                    }
                    Text(tx.counterpartyName ?? String(tx.counterpartyPublicKeyHex.prefix(16)))
                        .font(.footnote)
                    Text(dateString(tx.timestampUtc))
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                    if !tx.memo.isEmpty {
                        Text(tx.memo).font(.caption2).foregroundStyle(.secondary)
                    }
                }
            }
            .navigationTitle("History")
        }
    }

    private func statusColor(_ status: SyncStatus) -> Color {
        switch status {
        case .confirmed: return .green
        case .pending: return .orange
        case .conflicted: return .red
        case .rejected: return .gray
        }
    }

    private func dateString(_ micros: Int64) -> String {
        let date = Date(timeIntervalSince1970: TimeInterval(micros) / 1_000_000)
        let fmt = DateFormatter()
        fmt.dateFormat = "yyyy-MM-dd HH:mm"
        return fmt.string(from: date)
    }
}
