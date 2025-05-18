use mrz_core::parser::parse_any;
use mrz_core::ParsedMRZ;

#[test]
fn test_valid_icao() {
    let line1 = b"P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<";
    let line2 = b"L898902C36UTO7408122F1204159ZE184226B<<<<<<<";

    let result = parse_any(&[line1, line2]);
    assert!(matches!(result, Ok(ParsedMRZ::ICAO(_))));
}
