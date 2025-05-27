use mrz_core::parser::parse_any;
use mrz_core::{MRZChecksumError, MRZParseError};
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

#[test]
fn test_td3_with_missing_final_check_digit() {
    let line1 = b"PPUTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<";
    let line2 = b"L898902C36UTO7408122F1204159ZE184226B<<<<<1<"; // '<' for final check

    let result = parse_any(&[line1, line2]);
    assert!(
        matches!(result, Ok(ParsedMRZ::MrzIcaoTd3(_))),
        "Expected ParsedMRZ::MrzIcaoTd3 with missing final check, got {:?}",
        result
    );
    if let Ok(ParsedMRZ::MrzIcaoTd3(mrz)) = result {
        assert_eq!(
            mrz.is_final_check_valid(),
            None,
            "Final check digit should be treated as absent"
        );
    }
}

#[test]
fn test_td3_with_ocr_error_in_expiry_date() {
    let line1 = b"PPUTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<";
    let line2 = b"L898902C36UTO7408122F12O4159ZE184226B<<<<<10"; // 'O' instead of '0' in expiry

    let result = parse_any(&[line1, line2]);
    assert!(
        matches!(
            result,
            Err(MRZParseError::InvalidChecksumField(
                MRZChecksumError::ExpiryDate
            ))
        ),
        "Expected InvalidChecksumField(ExpiryDate), got {:?}",
        result
    );
}
#[test]
fn test_td3_with_common_ocr_errors_in_document_number() {
    let line1 = b"PPUTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<";
    let variants = [
        // 'O' instead of '0'
        b"L8989O2C36UTO7408122F1204159ZE184226B<<<<<10",
    ];

    for &line2 in &variants {
        let result = parse_any(&[line1, line2]);
        assert!(
            matches!(
                result,
                Err(MRZParseError::InvalidChecksumField(
                    MRZChecksumError::DocumentNumber
                ))
            ),
            "Expected InvalidChecksumField(DocumentNumber) for variant: {:?}, got {:?}",
            std::str::from_utf8(line2),
            result
        );
    }
}
#[test]
fn test_td3_with_ocr_s_instead_of_5_in_document_number() {
    // Document number XSZ9876546 is an OCR error for X5Z9876546 (which has check digit '6')
    let line1 = b"PPUTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<";
    let line2 = b"XSZ9876546UTO7408122F1204159ZE184226B<<<<<10";

    let result = parse_any(&[line1, line2]);
    assert!(
        matches!(
            result,
            Err(MRZParseError::InvalidChecksumField(
                MRZChecksumError::DocumentNumber
            ))
        ),
        "Expected InvalidChecksumField(DocumentNumber) with S->5 OCR error, got {:?}",
        result
    );
}
#[test]
fn test_td3_with_ocr_i_instead_of_1_in_document_number() {
    // Document number XIZ9876544 is an OCR error for X1Z9876544 (which has check digit '4')
    let line1 = b"PPUTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<";
    let line2 = b"XIZ9876544UTO7408122F1204159ZE184226B<<<<<10";

    let result = parse_any(&[line1, line2]);
    assert!(
        matches!(
            result,
            Err(MRZParseError::InvalidChecksumField(
                MRZChecksumError::DocumentNumber
            ))
        ),
        "Expected InvalidChecksumField(DocumentNumber) with I->1 OCR error, got {:?}",
        result
    );
}
