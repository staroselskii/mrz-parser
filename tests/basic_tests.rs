#[test]
fn test_parse_mrz() {
    let result = mrz_parser::parse_mrz("P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<");
    assert!(result.is_ok());
}
