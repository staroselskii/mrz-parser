//! Validation utilities for MRZ fields.

/// Checks if the input field contains only valid MRZ alphanumeric characters.
pub fn is_valid_alphanumeric(field: &str) -> bool {
    field
        .chars()
        .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '<')
}

/// Checks if the input field contains only numeric digits.
pub fn is_valid_numeric(field: &str) -> bool {
    field.chars().all(|c| c.is_ascii_digit())
}

/// Normalizes the input field by trimming spaces and converting to uppercase.
pub fn normalize_field(field: &str) -> String {
    field.trim().to_uppercase()
}
