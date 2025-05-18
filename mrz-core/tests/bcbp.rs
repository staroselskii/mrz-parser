use mrz_core::parser::parse_any;
use mrz_core::ParsedMRZ;

#[test]
fn test_valid_bcbp() {
    let line = b"M1SMITH/JOHN           AA1234 JFKLAX12C3";
    let result = parse_any(&[line]);
    assert!(matches!(result, Ok(ParsedMRZ::BCBP(_))));
}
