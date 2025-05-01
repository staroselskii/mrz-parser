use mrz_parser::{parse_mrz, parse_mrz_with_options, ParseOptions};

fn assert_valid_mrz(lines: [&str; 2]) {
    parse_mrz(&lines).expect("Expected valid MRZ but got error");
}

#[test]
fn test_valid_td3_passports() {
    let lines = [
        "P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<",
        "L898902C36UTO7408122F1204159ZE184226B<<<<<10",
    ];
    println!("Line 1 length: {}", lines[0].len());
    assert_eq!(lines[0].len(), 44, "Line 1 is not 44 chars");
    assert_eq!(lines[1].len(), 44, "Line 2 is not 44 chars");
    assert_valid_mrz(lines);
}

#[test]
fn test_invalid_td3_line_length() {
    let lines = [
        "P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<", // <-- too short
        "L898902C36UTO7408122F1204159ZE184226B<<<<<10",
    ];
    assert_eq!(lines[0].len(), 43);
    assert!(parse_mrz(&lines).is_err());
}

#[test]
fn test_invalid_td3_illegal_characters() {
    let lines = [
        "P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<<<<", // <-- good line
        "L8989@2C36UTO7408122F1204159ZE184226B<<<<<10",    // <-- illegal '@'
    ];
    assert!(parse_mrz(&lines).is_err());
}

#[test]
fn test_invalid_td3_checksum_errors() {
    let lines = [
        "P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<<<<", // <-- good line
        "L898902C36UTO7408122F1204159ZE184226B<<<<<11",    // <-- broken final checksum
    ];
    assert!(parse_mrz_with_options(&lines, ParseOptions { validate_final_checksum: true }).is_err());
}

#[test]
fn test_strict_final_checksum_validation() {
    let lines = [
        "P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<",
        "L898902C36UTO7408122F1204159ZE184226B<<<<<10",
    ];

    // Normal parse: should succeed
    assert!(parse_mrz_with_options(&lines, ParseOptions { validate_final_checksum: true }).is_ok());

    // Strict parse: should succeed
    let options = ParseOptions {
        validate_final_checksum: true,
    };
    assert!(parse_mrz_with_options(&lines, options).is_ok());
}

#[test]
fn test_strict_final_checksum_failure() {
    let broken_lines = [
        "P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<",
        "L898902C36UTO7408122F1204159ZE184226B<<<<<11",
    ];

    // Normal parse: should succeed (no strict checking)
    assert!(parse_mrz(&broken_lines).is_ok());

    // Strict parse: should fail
    let options = ParseOptions {
        validate_final_checksum: true,
    };
    assert!(parse_mrz_with_options(&broken_lines, options).is_err());
}

