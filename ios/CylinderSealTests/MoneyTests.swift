import XCTest
@testable import CylinderSeal

final class MoneyTests: XCTestCase {
    func testOneOwcFormats() {
        XCTAssertEqual(MoneyFormat.format(1_000_000, currency: "IQD"), "1.00 IQD")
    }

    func testFractionalAmount() {
        XCTAssertEqual(MoneyFormat.format(1_500_000, currency: "IQD"), "1.50 IQD")
    }

    func testThousandSeparator() {
        XCTAssertEqual(MoneyFormat.format(1_000_000_000, currency: "IQD"), "1,000.00 IQD")
    }

    func testParseInteger() {
        XCTAssertEqual(MoneyFormat.parse("5"), 5_000_000)
    }

    func testParseFractional() {
        XCTAssertEqual(MoneyFormat.parse("1.5"), 1_500_000)
    }

    func testParseStripsCommas() {
        XCTAssertEqual(MoneyFormat.parse("1,000"), 1_000_000_000)
    }

    func testParseRejectsGarbage() {
        XCTAssertNil(MoneyFormat.parse("abc"))
    }
}
