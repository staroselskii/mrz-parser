use mrz_core::{parser::parse_any, MRZParseError, MrzIcaoCommonFields, ParsedMRZ};

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
            Some(true),
            "Final check unexpectedly missing or incorrect"
        );
        assert!(
            mrz.is_document_number_valid(),
            "Document number check failed"
        );
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
    }
}
