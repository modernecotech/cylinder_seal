import XCTest
@testable import CylinderSeal

/// Exercises the Swift facade over the UniFFI-generated `cs_mobile_core`
/// bindings. These tests require the xcframework to be built and linked
/// — see `ios/scripts/build-rust-xcframework.sh`. They will fail to link
/// if the UniFFI scaffolding isn't in place; that's intentional, since
/// the whole point of the facade is to exercise the real Rust core.
final class MobileCoreFacadeTests: XCTestCase {
    func testKeypairGenerationProducesDistinctKeys() throws {
        let kp1 = try MobileCore.generateKeypair()
        let kp2 = try MobileCore.generateKeypair()
        XCTAssertEqual(kp1.publicKey.count, 32)
        XCTAssertEqual(kp1.privateKey.count, 32)
        XCTAssertNotEqual(kp1.publicKey, kp2.publicKey)
        XCTAssertNotEqual(kp1.privateKey, kp2.privateKey)
    }

    func testBlake2bOutputIs32Bytes() {
        let hash = MobileCore.blake2b256(Data("hello".utf8))
        XCTAssertEqual(hash.count, 32)
    }

    func testBlake2bIsDeterministic() {
        let a = MobileCore.blake2b256(Data("same input".utf8))
        let b = MobileCore.blake2b256(Data("same input".utf8))
        XCTAssertEqual(a, b)
    }

    func testUserIdIsValidUuid() throws {
        let kp = try MobileCore.generateKeypair()
        let uid = try MobileCore.userIdFromPublicKey(kp.publicKey)
        XCTAssertNotNil(UUID(uuidString: uid))
    }

    func testSignedTransactionRoundtrip() throws {
        let sender = try MobileCore.generateKeypair()
        let recipient = try MobileCore.generateKeypair()

        let input = MobileCore.TransactionInput(
            fromPublicKey: sender.publicKey,
            toPublicKey: recipient.publicKey,
            amountMicroOwc: 1_000_000,
            currencyContext: "IQD",
            fxRateSnapshot: "1.0",
            channel: .nfc,
            memo: "test",
            deviceId: UUID().uuidString,
            previousNonce: Data(count: 32),
            currentNonce: Data(repeating: 1, count: 32),
            latitude: 33.3152,
            longitude: 44.3661,
            locationAccuracyMeters: 10,
            locationSource: .gps
        )
        let cbor = try MobileCore.buildAndSignTransaction(input, privateKey: sender.privateKey)

        let view = try MobileCore.decodeTransaction(cbor)
        XCTAssertEqual(view.amountMicroOwc, 1_000_000)
        XCTAssertTrue(view.signatureValid, "Spec: signed tx must verify after CBOR roundtrip")
        XCTAssertEqual(view.channel, .nfc)
    }

    func testQrEncodeDecodeRoundtrip() throws {
        let payload = Data([0xde, 0xad, 0xbe, 0xef])
        let qr = try MobileCore.encodeQrPayload(payload)
        XCTAssertTrue(qr.hasPrefix("CS1:"))
        let decoded = try MobileCore.decodeQrPayload(qr)
        XCTAssertEqual(decoded, payload)
    }

    func testNfcApduFramesStartWithSelect() throws {
        let payload = Data(repeating: 0x42, count: 500)
        let frames = try MobileCore.buildNfcApdus(payload)
        XCTAssertGreaterThan(frames.count, 0)
        // First frame is SELECT AID: CLA=00, INS=A4.
        XCTAssertEqual(frames[0][0], 0x00)
        XCTAssertEqual(frames[0][1], 0xA4)
    }

    func testDeriveNextNonceIsDeterministic() throws {
        let prev = Data(count: 32)
        let seed = Data(repeating: 0xAB, count: 32)
        let a = try MobileCore.deriveNextNonce(prev: prev, hardwareSeed: seed, counter: 1)
        let b = try MobileCore.deriveNextNonce(prev: prev, hardwareSeed: seed, counter: 1)
        XCTAssertEqual(a, b, "Spec: RFC 6979 nonce derivation must be deterministic")
    }
}
