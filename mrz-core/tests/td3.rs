use mrz_core::parser::parse_any;
use mrz_core::{MrzIcaoCommonFields, ParsedMRZ};

#[test]
fn test_valid_td3_wo_final_checksum() {
    let line1 = b"P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<";
    let line2 = b"L898902C36UTO7408122F1204159ZE184226B<<<<<<<";

    let result = parse_any(&[line1, line2]);
    assert!(matches!(result, Ok(ParsedMRZ::MrzIcaoTd3(_))));
}

#[test]
fn test_valid_td3_with_final_checksum() {
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
            mrz.is_document_number_valid(),
            "Document number check should have passed"
        );
        assert!(
            mrz.is_birth_date_valid(),
            "Birth date check should have passed"
        );
        assert!(
            mrz.is_expiry_date_valid(),
            "Expiry date check should have passed"
        );
        assert_eq!(
            mrz.is_final_check_valid(),
            Some(true),
            "Final check digit should be present and correct"
        );
    }
}

#[test]
fn test_invalid_td3_final_checksum() {
    // Intentionally corrupted check digit in document number
    let line1 = b"PPUTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<";
    let line2 = b"L898902C36UTO7408122F1204159ZE184226B<<<<<11"; // 11 instead of 10

    let result = parse_any(&[line1, line2]);
    if let Ok(ParsedMRZ::MrzIcaoTd3(mrz)) = result {
        assert_eq!(
            mrz.is_final_check_valid(),
            Some(false),
            "Final check digit should be present and incorrect"
        );
        assert_eq!(mrz.document_number(), "L898902C3");
        assert_eq!(mrz.surname().to_string(), "ERIKSSON");
        assert_eq!(mrz.given_names().to_string(), "ANNA MARIA");
        assert_eq!(core::str::from_utf8(mrz.nationality()).unwrap(), "UTO");
        assert_eq!(core::str::from_utf8(mrz.birth_date()).unwrap(), "740812");
        assert_eq!(core::str::from_utf8(mrz.expiry_date()).unwrap(), "120415");
        assert_eq!(mrz.sex(), b'F');
    }
}

#[test]
fn test_invalid_td3_checksums() {
    // Intentionally corrupted check digit in document number
    let line1 = b"PPUTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<";
    let line2 = b"L898902C37UTO7408121F1204158ZE184226B<<<<<10"; // 1 instead of 2, 7 instead of 6, 8 instead of 9

    let result = parse_any(&[line1, line2]);
    if let Ok(ParsedMRZ::MrzIcaoTd3(mrz)) = result {
        assert!(
            !mrz.is_document_number_valid(),
            "Document number check should have failed"
        );
        assert!(
            !mrz.is_birth_date_valid(),
            "Birth date check should have failed"
        );
        assert!(
            !mrz.is_expiry_date_valid(),
            "Expiry date check should have failed"
        );
        assert_eq!(
            mrz.is_final_check_valid(),
            Some(false),
            "Final check digit should be present and incorrect"
        );
    }
}
