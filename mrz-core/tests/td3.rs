use mrz_core::parser::parse_any;
use mrz_core::ParsedMRZ;

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
            mrz.document_number_check_valid,
            "Document number check should have passed"
        );
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
fn test_invalid_td3_final_checksum() {
    // Intentionally corrupted check digit in document number
    let line1 = b"PPUTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<";
    let line2 = b"L898902C36UTO7408122F1204159ZE184226B<<<<<11"; // 11 instead of 10

    let result = parse_any(&[line1, line2]);
    if let Ok(ParsedMRZ::MrzIcaoTd3(mrz)) = result {
        assert_eq!(
            mrz.final_check_valid,
            Some(false),
            "Final check digit should be present and incorrect"
        );
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
            !mrz.document_number_check_valid,
            "Document number check should have failed"
        );
        assert!(
            !mrz.birth_date_check_valid
            ,
            "Birth date check should have failed"
        );
        assert!(
            !mrz.expiry_date_check_valid
            ,
            "Expiry date check should have failed"
        );
        assert_eq!(
            mrz.final_check_valid,
            Some(false),
            "Final check digit should be present and incorrect"
        );
    }
}
