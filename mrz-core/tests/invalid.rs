use mrz_core::parser::parse_any;
use mrz_core::{ParsedMRZ, MRZParseError};

#[test]
fn test_invalid_format() {
    let line = b"THISISNOTVALIDMRZDATA";
    let result = parse_any(&[line]);
    assert!(matches!(result, Err(MRZParseError::UnknownFormat)));
}
