use mrz_core::checked_field::CheckedField;
use mrz_core::MRZChecksumError;

#[test]
fn test_is_valid_and_has_error() {
    let valid = CheckedField::new(42, None, b'<');
    let invalid = CheckedField::new(42, Some(MRZChecksumError::Final), b'<');

    assert!(valid.is_valid());
    assert!(!valid.has_error());
    assert!(!invalid.is_valid());
    assert!(invalid.has_error());
}

#[test]
fn test_as_ref() {
    let field = CheckedField::new(String::from("ABC123"), None, b'<');
    let field_ref = field.as_ref();

    assert_eq!(field.value(), *field_ref.value());
    assert!(field_ref.error().is_none());
}

#[test]
fn test_map_error() {
    let original = CheckedField::new(99, Some(MRZChecksumError::Final), b'<');
    let mapped = original.clone().map_error(|_| MRZChecksumError::Final);

    assert_eq!(original.value(), mapped.value());
    assert_eq!(original.error(), mapped.error());
}

#[test]
fn test_display_checked_field() {
    use crate::MRZChecksumError;

    let valid = CheckedField::new("ABC123", None, b'<');
    assert_eq!(format!("{}", valid), "ABC123");

    let invalid = CheckedField::new("XYZ", Some(MRZChecksumError::DocumentNumber), b'<');
    assert_eq!(format!("{}", invalid), "XYZ (invalid: DocumentNumber)");
}
