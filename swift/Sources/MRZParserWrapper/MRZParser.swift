import Foundation
import MRZParserFFI


private let mrzDateFormatter: DateFormatter = {
    let formatter = DateFormatter()
    formatter.dateFormat = "yyyy-MM-dd"
    formatter.locale = Locale(identifier: "en_US_POSIX")
    formatter.timeZone = TimeZone(secondsFromGMT: 0)
    return formatter
}()

private func parseMRZDate(_ string: String) -> Date {
    return mrzDateFormatter.date(from: string) ?? .distantPast
}

public enum MRZDocumentType {
    case passport
    case idCard
    case visa
    case unknown
}

public struct MRZParsed {
    public let documentNumber: String
    public let name: String
    public let givenNames: String
    public let surname: String
    public let nationality: String
    public let birthDate: Date
    public let expiryDate: Date
    public let documentType: MRZDocumentType
    public let issuingState: String
    public let sex: String
    public let optionalData1: String
    public let optionalData2: String
}

public enum MRZParserError: Error {
    case invalidInput(String)
    case parseFailed(String)
}

/// Swift-native wrapper for the MRZ parser
public struct MRZParser {
    public init() {}

    public func parse(lines: [String]) throws -> MRZParsed {
        do {
            let result = try MRZParserFFI.parseLines(lines: lines)
            return MRZParsed(
                documentNumber: result.documentNumber,
                name: result.name,
                givenNames: result.givenNames,
                surname: result.surname,
                nationality: result.nationality,
                birthDate: parseMRZDate(result.birthDate),
                expiryDate: parseMRZDate(result.expiryDate),
                documentType: mapDocType(result.documentType),
                issuingState: result.issuingState,
                sex: result.sex,
                optionalData1: result.optionalData1,
                optionalData2: result.optionalData2
            )
        } catch let err as MRZParserFFI.MrzParseError {
            throw MRZParserError.parseFailed(err.localizedDescription)
        } catch {
            throw MRZParserError.invalidInput("Unknown error")
        }
    }

	private func mapDocType(_ type: String) -> MRZDocumentType {
	    switch type.uppercased() {
	    case "P": return .passport
	    case "I": return .idCard
	    case "V": return .visa
	    default: return .unknown
	    }
}
}
