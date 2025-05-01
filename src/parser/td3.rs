//! Parser for MRZ TD3 (passport) format.
//!
//! Provides strict parsing with validation and checksum verification.

use crate::validation::checksum::calculate_mrz_checksum;
use crate::validation::field_validators::{
    is_valid_alphanumeric, is_valid_numeric, normalize_field,
};
use crate::{MRZData, MRZParseError, ParseOptions};

/// Strict parser for TD3 MRZ (Passport) format.
///
/// Validates each field strictly, including checksum verification.
pub fn parse_td3_mrz_strict(
    lines: &[&str],
    options: ParseOptions,
) -> Result<MRZData, MRZParseError> {
    if lines.len() != 2 {
        return Err(MRZParseError::InvalidFormat);
    }

    let line1 = lines[0];
    let line2 = lines[1];

    if line1.len() != 44 || line2.len() != 44 {
        return Err(MRZParseError::InvalidFormat);
    }

    let document_type = &line1[0..1];
    if !is_valid_alphanumeric(document_type) {
        return Err(MRZParseError::FieldError("Invalid document type"));
    }

    let issuing_country = &line1[2..5];
    if !issuing_country.chars().all(|c| c.is_ascii_uppercase()) {
        return Err(MRZParseError::FieldError("Invalid issuing country"));
    }

    let names = parse_names(&line1[5..44])?;

    let passport_number_raw = &line2[0..9];
    let passport_number_checksum = line2
        .chars()
        .nth(9)
        .ok_or(MRZParseError::FieldError("Missing passport checksum"))?;
    let passport_number = validate_passport_number(passport_number_raw, passport_number_checksum)?;

    let nationality = &line2[10..13];
    if !nationality.chars().all(|c| c.is_ascii_uppercase()) {
        return Err(MRZParseError::FieldError("Invalid nationality"));
    }

    let birth_date_raw = &line2[13..19];
    let birth_date_checksum = line2
        .chars()
        .nth(19)
        .ok_or(MRZParseError::FieldError("Missing birth date checksum"))?;
    validate_numeric_field_with_checksum(birth_date_raw, birth_date_checksum)?;

    let sex = &line2[20..21];
    if !(sex == "M" || sex == "F" || sex == "<") {
        return Err(MRZParseError::FieldError("Invalid sex field"));
    }

    let expiry_date_raw = &line2[21..27];
    let expiry_date_checksum = line2
        .chars()
        .nth(27)
        .ok_or(MRZParseError::FieldError("Missing expiry date checksum"))?;
    validate_numeric_field_with_checksum(expiry_date_raw, expiry_date_checksum)?;

    if options.validate_final_checksum {
        validate_final_checksum(line2)?;
    }

    Ok(MRZData {
        document_type: document_type.to_string(),
        issuing_country: issuing_country.to_string(),
        names,
        passport_number,
        nationality: nationality.to_string(),
        birth_date: normalize_field(birth_date_raw),
        sex: sex.to_string(),
        expiry_date: normalize_field(expiry_date_raw),
    })
}

fn parse_names(name_field: &str) -> Result<(String, String), MRZParseError> {
    let mut parts = name_field.splitn(2, "<<");
    let surname = parts
        .next()
        .ok_or(MRZParseError::FieldError("Missing surname"))?;
    let given = parts
        .next()
        .ok_or(MRZParseError::FieldError("Missing given names"))?;

    let surname = surname.replace('<', " ").trim().to_string();
    let given_names = given.replace('<', " ").trim().to_string();

    Ok((surname, given_names))
}

fn validate_passport_number(field: &str, checksum_char: char) -> Result<String, MRZParseError> {
    if !is_valid_alphanumeric(field) {
        return Err(MRZParseError::FieldError(
            "Invalid passport number characters",
        ));
    }
    let expected_checksum = checksum_char.to_digit(10).ok_or(MRZParseError::FieldError(
        "Invalid passport number checksum character",
    ))?;
    if calculate_mrz_checksum(field) != expected_checksum {
        return Err(MRZParseError::FieldError(
            "Passport number checksum mismatch",
        ));
    }
    Ok(normalize_field(field))
}

fn validate_numeric_field_with_checksum(
    field: &str,
    checksum_char: char,
) -> Result<(), MRZParseError> {
    if !is_valid_numeric(field) {
        return Err(MRZParseError::FieldError(
            "Invalid numeric field characters",
        ));
    }
    let expected_checksum = checksum_char
        .to_digit(10)
        .ok_or(MRZParseError::FieldError("Invalid checksum character"))?;
    if calculate_mrz_checksum(field) != expected_checksum {
        return Err(MRZParseError::FieldError("Field checksum mismatch"));
    }
    Ok(())
}

/// Validates the final overall MRZ checksum from TD3 Line 2.
fn validate_final_checksum(line2: &str) -> Result<(), MRZParseError> {
    let mut data = String::new();
    data.push_str(&line2[0..10]); // passport number + checksum
    data.push_str(&line2[13..20]); // birthdate + checksum
    data.push_str(&line2[21..43]); // expiry date + personal number

    let expected_checksum = line2
        .chars()
        .nth(43)
        .ok_or(MRZParseError::FieldError(
            "Missing final checksum character",
        ))?
        .to_digit(10)
        .ok_or(MRZParseError::FieldError(
            "Invalid final checksum character",
        ))?;

    let calculated = calculate_mrz_checksum(&data);

    if calculated == expected_checksum {
        Ok(())
    } else {
        Err(MRZParseError::FieldError("Final checksum mismatch"))
    }
}
