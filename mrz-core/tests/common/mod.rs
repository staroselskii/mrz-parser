use mrz_core::MrzIcaoCommonFields;

pub fn assert_checksum_matches<T: MrzIcaoCommonFields>(mrz: &T) {
    assert_eq!(
        mrz.is_document_number_valid(),
        true,
        "Document number checksum failed"
    );
    assert_eq!(
        mrz.is_birth_date_valid(),
        true,
        "Birth date checksum failed"
    );
    assert_eq!(
        mrz.is_expiry_date_valid(),
        true,
        "Expiry date checksum failed"
    );

    if let Some(valid) = mrz.is_final_check_valid() {
        assert!(valid, "Final checksum failed");
    }
}
