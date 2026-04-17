import Foundation

/// Conversions between internal micro-OWC (Int64) and human-readable IQD
/// strings. 1 OWC = 1_000_000 micro-OWC.
enum MoneyFormat {
    static let microPerUnit: Int64 = 1_000_000

    static func format(_ microOwc: Int64, currency: String = "IQD") -> String {
        let whole = microOwc / microPerUnit
        let frac = abs(microOwc % microPerUnit)
        let fracDigits = Int64(Double(frac) / Double(microPerUnit) * 100)
        let formatter = NumberFormatter()
        formatter.numberStyle = .decimal
        let wholeStr = formatter.string(from: whole as NSNumber) ?? "\(whole)"
        return String(format: "%@.%02d %@", wholeStr, fracDigits, currency)
    }

    static func parse(_ s: String) -> Int64? {
        let trimmed = s.replacingOccurrences(of: ",", with: "").trimmingCharacters(in: .whitespaces)
        let parts = trimmed.split(separator: ".")
        switch parts.count {
        case 1:
            return Int64(parts[0]).map { $0 * microPerUnit }
        case 2:
            guard let whole = Int64(parts[0]) else { return nil }
            let fracSrc = String(parts[1])
            let padded = fracSrc.padding(toLength: 6, withPad: "0", startingAt: 0)
            let capped = String(padded.prefix(6))
            guard let frac = Int64(capped) else { return nil }
            return whole * microPerUnit + frac
        default:
            return nil
        }
    }
}
