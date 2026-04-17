import Foundation

/// Swift facade over the UniFFI-generated bindings for `cs-mobile-core`.
///
/// The UniFFI Swift bindings (module `cs_mobile_core`) are produced from
/// `crates/cs-mobile-core/src/cs_mobile_core.udl` by running:
///
///     uniffi-bindgen-swift crates/cs-mobile-core/src/cs_mobile_core.udl \
///         ios/CylinderSealCore
///
/// and placing the generated Swift + header files inside
/// `ios/CylinderSealCore/`. The native code itself ships as an
/// `xcframework` built by `scripts/build-rust-xcframework.sh`.
///
/// Feature modules should import this file — never the raw `uniffi`
/// module directly. That lets us swap generator versions without touching
/// every consumer.

#if canImport(CylinderSealCore)
import CylinderSealCore
#endif

enum MobileCoreError: Error, LocalizedError {
    case underlying(String)

    var errorDescription: String? {
        switch self {
        case .underlying(let msg): return msg
        }
    }
}

enum MobileCore {
    struct Keypair: Equatable {
        let publicKey: Data
        let privateKey: Data
    }

    struct TransactionInput {
        let fromPublicKey: Data
        let toPublicKey: Data
        let amountMicroOwc: Int64
        let currencyContext: String
        let fxRateSnapshot: String
        let channel: PaymentChannel
        let memo: String
        let deviceId: String
        let previousNonce: Data
        let currentNonce: Data
        let latitude: Double
        let longitude: Double
        let locationAccuracyMeters: Int32
        let locationSource: LocationSource
    }

    struct TransactionView: Equatable {
        let transactionId: String
        let fromPublicKey: Data
        let toPublicKey: Data
        let amountMicroOwc: Int64
        let currencyContext: String
        let timestampUtc: Int64
        let memo: String
        let channel: PaymentChannel
        let deviceId: String
        let signatureValid: Bool
    }

    // MARK: - Public surface

    static func generateKeypair() throws -> Keypair {
        let ffi = try callThrowing { try cs_mobile_core.generateKeypair() }
        return Keypair(publicKey: Data(ffi.publicKey), privateKey: Data(ffi.privateKey))
    }

    static func blake2b256(_ data: Data) -> Data {
        Data(cs_mobile_core.blake2b256(data: [UInt8](data)))
    }

    static func userIdFromPublicKey(_ publicKey: Data) throws -> String {
        try callThrowing { try cs_mobile_core.userIdFromPublicKey(publicKey: [UInt8](publicKey)) }
    }

    static func buildAndSignTransaction(_ input: TransactionInput, privateKey: Data) throws -> Data {
        let ffi = cs_mobile_core.TransactionInput(
            fromPublicKey: [UInt8](input.fromPublicKey),
            toPublicKey: [UInt8](input.toPublicKey),
            amountMicroOwc: input.amountMicroOwc,
            currencyContext: input.currencyContext,
            fxRateSnapshot: input.fxRateSnapshot,
            channel: Int32(input.channel.rawValue),
            memo: input.memo,
            deviceId: input.deviceId,
            previousNonce: [UInt8](input.previousNonce),
            currentNonce: [UInt8](input.currentNonce),
            latitude: input.latitude,
            longitude: input.longitude,
            locationAccuracyMeters: input.locationAccuracyMeters,
            locationSource: Int32(input.locationSource.rawValue)
        )
        let out = try callThrowing {
            try cs_mobile_core.buildAndSignTransaction(input: ffi, privateKey: [UInt8](privateKey))
        }
        return Data(out)
    }

    static func decodeTransaction(_ cbor: Data) throws -> TransactionView {
        let view = try callThrowing { try cs_mobile_core.decodeTransaction(cbor: [UInt8](cbor)) }
        return TransactionView(
            transactionId: view.transactionId,
            fromPublicKey: Data(view.fromPublicKey),
            toPublicKey: Data(view.toPublicKey),
            amountMicroOwc: view.amountMicroOwc,
            currencyContext: view.currencyContext,
            timestampUtc: view.timestampUtc,
            memo: view.memo,
            channel: PaymentChannel(rawValue: Int(view.channel)) ?? .online,
            deviceId: view.deviceId,
            signatureValid: view.signatureValid
        )
    }

    static func deriveNextNonce(prev: Data, hardwareSeed: Data, counter: UInt64) throws -> Data {
        let out = try callThrowing {
            try cs_mobile_core.deriveNextNonce(
                prevNonce: [UInt8](prev),
                hardwareSeed: [UInt8](hardwareSeed),
                counter: counter
            )
        }
        return Data(out)
    }

    static func encodeQrPayload(_ cbor: Data) throws -> String {
        try callThrowing { try cs_mobile_core.encodeQrPayload(cbor: [UInt8](cbor)) }
    }

    static func decodeQrPayload(_ qr: String) throws -> Data {
        let out = try callThrowing { try cs_mobile_core.decodeQrPayload(qr: qr) }
        return Data(out)
    }

    static func buildNfcApdus(_ cbor: Data) throws -> [Data] {
        let frames = try callThrowing { try cs_mobile_core.buildNfcApdus(cbor: [UInt8](cbor)) }
        return frames.map { Data($0) }
    }

    // MARK: - Error translation

    private static func callThrowing<T>(_ work: () throws -> T) throws -> T {
        do {
            return try work()
        } catch let e {
            throw MobileCoreError.underlying(String(describing: e))
        }
    }
}
