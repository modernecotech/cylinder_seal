import Foundation

extension Data {
    /// Encode to lowercase hex.
    func toHex() -> String {
        map { String(format: "%02x", $0) }.joined()
    }

    /// Decode a hex string (optional "0x" prefix) into `Data`.
    init?(hex: String) {
        let clean = hex.hasPrefix("0x") ? String(hex.dropFirst(2)) : hex
        guard clean.count % 2 == 0 else { return nil }
        var bytes = Data()
        bytes.reserveCapacity(clean.count / 2)
        var idx = clean.startIndex
        while idx < clean.endIndex {
            let next = clean.index(idx, offsetBy: 2)
            guard let b = UInt8(clean[idx..<next], radix: 16) else { return nil }
            bytes.append(b)
            idx = next
        }
        self = bytes
    }
}
