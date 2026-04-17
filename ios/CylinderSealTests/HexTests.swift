import XCTest
@testable import CylinderSeal

final class HexTests: XCTestCase {
    func testEmptyRoundtrip() {
        let data = Data()
        XCTAssertEqual(data.toHex(), "")
        XCTAssertEqual(Data(hex: ""), Data())
    }

    func testLowercaseHexEncoding() {
        let data = Data([0x0f, 0xff])
        XCTAssertEqual(data.toHex(), "0fff")
    }

    func testRoundtrip() {
        let bytes = Data([0x00, 0x7f, 0xff, 0x10, 0x20])
        let hex = bytes.toHex()
        let back = Data(hex: hex)
        XCTAssertEqual(back, bytes)
    }

    func testAcceptsOxPrefix() {
        XCTAssertEqual(Data(hex: "0xaabb"), Data([0xaa, 0xbb]))
    }

    func testRejectsOddLength() {
        XCTAssertNil(Data(hex: "abc"))
    }

    func testRejectsInvalidChars() {
        XCTAssertNil(Data(hex: "zzzz"))
    }
}
