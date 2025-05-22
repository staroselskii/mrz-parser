use mrz_core::parser::parse_any;
use mrz_core::ParsedMRZ;

#[test]
fn test_valid_td3() {
    let line1 = b"P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<";
    let line2 = b"L898902C36UTO7408122F1204159ZE184226B<<<<<<<";

    let result = parse_any(&[line1, line2]);
    assert!(matches!(result, Ok(ParsedMRZ::MrzIcaoTd3(_))));
}

#[test]
fn test_valid_td3_with_checksums() {
    // ICAO 9303 sample data
    let line1 = b"PPUTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<";
    let line2 = b"L898902C36UTO7408122F1204159ZE184226B<<<<<10";

    let result = parse_any(&[line1, line2]);
    assert!(
        matches!(result, Ok(ParsedMRZ::MrzIcaoTd3(_))),
        "Expected ParsedMRZ::MrzIcaoTd3, got {:?}",
        result
    );
    if let Ok(ParsedMRZ::MrzIcaoTd3(mrz)) = result {
        assert!(
            mrz.birth_date_check_valid,
            "Birth date check should have passed"
        );
        assert!(
            mrz.expiry_date_check_valid,
            "Expiry date check should have passed"
        );
        assert_eq!(
            mrz.final_check_valid,
            Some(true),
            "Final check digit should be present and correct"
        );
    }
}

#[test]
fn test_invalid_td3_checksum() {
    // Intentionally corrupted check digit in document number
    let line1 = b"P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<";
    let line2 = b"L898902C35UTO7408122F1204159ZE184226B<<<<<<<"; // Changed '6' to '5'

    let result = parse_any(&[line1, line2]);
    if let Ok(ParsedMRZ::MrzIcaoTd3(mrz)) = result {
        assert_eq!(
            mrz.final_check_valid, None,
            "Final check unexpectedly present"
        );
    }
}
