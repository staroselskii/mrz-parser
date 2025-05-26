import XCTest
@testable import MRZParserWrapper

final class MRZParserWrapperTests: XCTestCase {

    func testParseValidTD3MRZ() throws {
        let mrzLines = [
		  "I<UTOD231458907<<<<<<<<<<<<<<<",
		  "7408122F1204159UTO<<<<<<<<<<<6",
		  "ERIKSSON<<ANNA<MARIA<<<<<<<<<<"
		]



        let parser = MRZParser()
        let parsed = try parser.parse(lines: mrzLines)

        XCTAssertEqual(parsed.documentNumber, "D23145890")
        XCTAssertEqual(parsed.name, "ERIKSSON<<ANNA MARIA")
        XCTAssertEqual(parsed.nationality, "UTO")
        XCTAssertEqual(parsed.birthDate, parseMRZDate("1974-08-12"))
        XCTAssertEqual(parsed.expiryDate, parseMRZDate("2012-04-15"))
        XCTAssertEqual(parsed.documentType, .idCard)
        XCTAssertEqual(parsed.issuingState, "UTO")
        XCTAssertEqual(parsed.sex, "F")
        XCTAssertEqual(parsed.optionalData1, "<<<<<<<<<<<<<<<")
        XCTAssertEqual(parsed.optionalData2, "<<<<<<<<<<<")
}
}

// MARK: - Helpers

private func parseMRZDate(_ string: String) -> Date {
let formatter = DateFormatter()
formatter.dateFormat = "yyyy-MM-dd"
formatter.timeZone = TimeZone(secondsFromGMT: 0)
return formatter.date(from: string)!
}
