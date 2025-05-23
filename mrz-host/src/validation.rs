use mrz_core::{MRZChecksumError, MRZParseError, MrzIcaoCommonFields};

pub fn validate_common_fields(raw: &impl MrzIcaoCommonFields) -> Result<(), MRZParseError> {
    if !raw.is_birth_date_valid() {
        return Err(MRZParseError::InvalidChecksumField(
            MRZChecksumError::BirthDate,
        ));
    }
    if !raw.is_expiry_date_valid() {
        return Err(MRZParseError::InvalidChecksumField(
            MRZChecksumError::ExpiryDate,
        ));
    }
    if raw.is_final_check_valid() == Some(false) {
        return Err(MRZParseError::InvalidChecksumField(MRZChecksumError::Final));
    }
    Ok(())
}

pub fn validate_td1_fields(raw: &impl MrzIcaoCommonFields) -> Result<(), MRZParseError> {
    validate_common_fields(raw)?;
    if !raw.is_document_number_valid() {
        return Err(MRZParseError::InvalidChecksumField(
            MRZChecksumError::DocumentNumber,
        ));
    }
    Ok(())
}
