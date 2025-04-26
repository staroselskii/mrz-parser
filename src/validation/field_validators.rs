/// Validation utilities for MRZ fields.

/// Check if a field is alphanumeric (A-Z, 0-9, or '<')
pub fn is_valid_alphanumeric(field: &str) -> bool {
    field.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '<')
}

/// Check if a field is purely numeric (0-9)
pub fn is_valid_numeric(field: &str) -> bool {
    field.chars().all(|c| c.is_ascii_digit())
}

/// Normalize field by trimming and uppercasing
pub fn normalize_field(field: &str) -> String {
    field.trim().to_uppercase()
}
