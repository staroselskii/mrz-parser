use mrz_core::field_correction::correct_checked_field;
use mrz_core::MRZChecksumError;

#[test]
fn test_document_number_ocr_s_as_5() {
    use heapless::String;
    const MAX_FIELD_PERMUTATIONS: usize = 8;

    let field_str: &str = "D2314589O"; // Example field with 'O' that should be corrected to '0'
    let check_char: char = '7';

    let corrected = correct_checked_field::<9, MAX_FIELD_PERMUTATIONS, String<9>>(
        field_str,
        check_char,
        MAX_FIELD_PERMUTATIONS,
        MRZChecksumError::DocumentNumber,
    );

    assert!(corrected.is_ok(), "Correction failed: {corrected:?}");
    assert_eq!(
        corrected.unwrap().value().as_str(),
        "D23145890",
        "Corrected string mismatch"
    );
}
