use mrz_parser::validation::field_validators::*;
use mrz_parser::validation::checksum::calculate_mrz_checksum;

#[test]
fn test_alphanumeric_validation() {
    assert!(is_valid_alphanumeric("AB123<"));
    assert!(!is_valid_alphanumeric("ab123"));
}

#[test]
fn test_numeric_validation() {
    assert!(is_valid_numeric("123456"));
    assert!(!is_valid_numeric("12A456"));
}

#[test]
fn test_checksum_calculation() {
    let passport_number = "L898902C3";
    let checksum = calculate_mrz_checksum(passport_number);
    assert_eq!(checksum, 6); // Known correct checksum
}
