use mrz_core::{parser::parse_any, MRZParseError, MrzIcaoCommonFields, ParsedMRZ};

mod common;
use common::assert_checksum_matches;

#[test]
fn test_valid_td1_without_final_check() {
    let lines = [
        b"I<UTOD231458907<<<<<<<<<<<<<<<",
        b"7408122F1204159UTO<<<<<<<<<<<<",
        b"ERIKSSON<<ANNA<MARIA<<<<<<<<<<",
    ];
    let lines_ref: [&[u8]; 3] = [&lines[0][..], &lines[1][..], &lines[2][..]];
    let result = parse_any(&lines_ref);
    assert!(
        matches!(result, Ok(ParsedMRZ::MrzIcaoTd1(_))),
        "Expected ParsedMRZ::MrzIcaoTd1, got {:?}",
        result
    );
    if let Ok(ParsedMRZ::MrzIcaoTd1(mrz)) = result {
        assert!(mrz.is_birth_date_valid(), "Birth date check failed");
        assert!(mrz.is_expiry_date_valid(), "Expiry date check failed");
        assert_eq!(
            mrz.is_final_check_valid(),
            None,
            "Expected final check to be skipped"
        );
        assert!(
            mrz.is_document_number_valid(),
            "Document number check failed"
        );
        assert_eq!(mrz.given_names(), "ANNA MARIA", "Given names did not match");
        assert_eq!(mrz.surname(), "ERIKSSON", "Surnames did not match");
        assert_checksum_matches(&mrz);
    }
}

#[test]
fn test_invalid_td1_sample() {
    let lines = [
        b"I<UTOD231458907<<<<<<<<<<<<<<<",
        b"7408121F1204153UTO<<<<<<<<<<<<",
        b"ERIKSSON<<ANNA<MARIA<<<<<<<<<<",
    ];
    let lines_ref: [&[u8]; 3] = [&lines[0][..], &lines[1][..], &lines[2][..]];
    let result = parse_any(&lines_ref);
    assert!(
        matches!(result, Err(MRZParseError::InvalidChecksumField(_))),
        "Expected Err(MRZParseError::InvalidChecksumField(_)), got {:?}",
        result
    );
}

#[test]
fn test_valid_td1_with_final_check_1() {
    let lines = [
        b"I<UTOD231458907<<<<<<<<<<<<<<<",
        b"7408122F1204159UTO<<<<<<<<<<<6",
        b"ERIKSSON<<ANNA<MARIA<<<<<<<<<<",
    ];

    let lines_ref: [&[u8]; 3] = [&lines[0][..], &lines[1][..], &lines[2][..]];
    let result = parse_any(&lines_ref);
    assert!(
        matches!(result, Ok(ParsedMRZ::MrzIcaoTd1(_))),
        "Expected ParsedMRZ::MrzIcaoTd1, got {:?}",
        result
    );
    if let Ok(ParsedMRZ::MrzIcaoTd1(mrz)) = result {
        assert!(
            mrz.is_document_number_valid(),
            "Document number check failed"
        );
        assert!(mrz.is_birth_date_valid(), "Birth date check failed");
        assert!(mrz.is_expiry_date_valid(), "Expiry date check failed");
        assert_eq!(
            mrz.is_final_check_valid(),
            Some(true),
            "Final check missing or incorrect"
        );
        dbg!(&mrz);
        assert_eq!(mrz.given_names(), "ANNA MARIA", "Given names did not match");
        assert_eq!(mrz.surname(), "ERIKSSON", "Surnames did not match");
        assert_checksum_matches(&mrz);
    }
}
#[test]
fn test_valid_td1_with_final_check_2() {
    let lines = [
        b"I<YTOD231458907<<<<<<<<<<<<<<<",
        b"3407127M9507122YTO<<<<<<<<<<<2",
        b"ERIKSSON<<ANNA<MARIA<<<<<<<<<<",
    ];

    let lines_ref: [&[u8]; 3] = [&lines[0][..], &lines[1][..], &lines[2][..]];
    let result = parse_any(&lines_ref);
    assert!(
        matches!(result, Ok(ParsedMRZ::MrzIcaoTd1(_))),
        "Expected ParsedMRZ::MrzIcaoTd1, got {:?}",
        result
    );
    if let Ok(ParsedMRZ::MrzIcaoTd1(mrz)) = result {
        assert!(
            mrz.is_document_number_valid(),
            "Document number check failed"
        );
        assert!(mrz.is_birth_date_valid(), "Birth date check failed");
        assert!(mrz.is_expiry_date_valid(), "Expiry date check failed");
        assert!(
            mrz.is_birth_date_valid(),
            "Final check missing or incorrect"
        );
        assert_eq!(mrz.given_names(), "ANNA MARIA", "Given names did not match");
        assert_eq!(mrz.surname(), "ERIKSSON", "Surnames did not match");
        assert_checksum_matches(&mrz);
    }
}

#[test]
fn test_td1_document_number_with_ocr_substitution_o_instead_of_0() {
    let lines = [
        b"I<YTOD2314589O7<<<<<<<<<<<<<<<", // 'O' instead of '0'
        b"3407127M9507122YTO<<<<<<<<<<<2",
        b"ERIKSSON<<ANNA<MARIA<<<<<<<<<<",
    ];
    let lines_ref: [&[u8]; 3] = [&lines[0][..], &lines[1][..], &lines[2][..]];
    let result = parse_any(&lines_ref);
    assert!(
        result.is_ok(),
        "Expected successful parse with corrected OCR error, got {:?}",
        result
    );
}

#[test]
fn test_td1_document_number_with_ocr_substitution_5_instead_of_s() {
    let lines = [
        b"I<YTOD2314S8907<<<<<<<<<<<<<<<", // 'S' insread of '5'
        b"3407127M9507122YTO<<<<<<<<<<<2",
        b"ERIKSSON<<ANNA<MARIA<<<<<<<<<<",
    ];
    let lines_ref: [&[u8]; 3] = [&lines[0][..], &lines[1][..], &lines[2][..]];
    let result = parse_any(&lines_ref);
    assert!(
        result.is_ok(),
        "Expected successful parse with corrected OCR error, got {:?}",
        result
    );
}

#[test]
fn test_td1_with_ocr_error_in_birth_date() {
    let lines = [
        b"I<YTOD231458907<<<<<<<<<<<<<<<",
        b"34O7127M9507122YTO<<<<<<<<<<<2", // 'O' instead of '0' in birth date
        b"ERIKSSON<<ANNA<MARIA<<<<<<<<<<",
    ];
    let lines_ref: [&[u8]; 3] = [&lines[0][..], &lines[1][..], &lines[2][..]];
    let result = parse_any(&lines_ref);
    assert!(
        matches!(result, Ok(ParsedMRZ::MrzIcaoTd1(_))),
        "Expected successful parse with corrected birth date OCR error, got {:?}",
        result
    );
    if let Ok(ParsedMRZ::MrzIcaoTd1(mrz)) = result {
        assert!(
            mrz.is_birth_date_valid(),
            "Birth date should have been corrected"
        );
    }
}

#[test]
fn test_td1_with_missing_final_check_digit() {
    let lines = [
        b"I<UTOD231458907<<<<<<<<<<<<<<<",
        b"7408122F1204159UTO<<<<<<<<<<<<", // final check digit is '<' (absent)
        b"ERIKSSON<<ANNA<MARIA<<<<<<<<<<",
    ];
    let lines_ref = [&lines[0][..], &lines[1][..], &lines[2][..]];
    let result = parse_any(&lines_ref);
    assert!(
        matches!(result, Ok(ParsedMRZ::MrzIcaoTd1(_))),
        "Expected successful parse with missing final check digit, got {:?}",
        result
    );
    if let Ok(ParsedMRZ::MrzIcaoTd1(mrz)) = result {
        assert_eq!(
            mrz.is_final_check_valid(),
            None,
            "Expected final check to be skipped"
        );
        assert!(mrz.is_document_number_valid());
        assert!(mrz.is_birth_date_valid());
        assert!(mrz.is_expiry_date_valid());
        assert_checksum_matches(&mrz);
    }
}

#[test]
fn test_td1_with_ocr_error_in_expiry_date() {
    let lines = [
        b"I<UTOD231458907<<<<<<<<<<<<<<<",
        b"7408122F12O4159UTO<<<<<<<<<<<6", // 'O' instead of '0' in expiry date
        b"ERIKSSON<<ANNA<MARIA<<<<<<<<<<",
    ];
    let lines_ref = [&lines[0][..], &lines[1][..], &lines[2][..]];
    let result = parse_any(&lines_ref);
    assert!(
        matches!(result, Ok(ParsedMRZ::MrzIcaoTd1(_))),
        "Expected successful parse due to correctable OCR error, got {:?}",
        result
    );
    if let Ok(ParsedMRZ::MrzIcaoTd1(mrz)) = result {
        assert!(
            mrz.is_expiry_date_valid(),
            "Expiry date should have been corrected"
        );
    }
}
