import Foundation

let result = try parseLines(["P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<", "L898902C36UTO7408122F1204159ZE184226B<<<<<10"])

print("Parsed document number: \(result.documentNumber)")
