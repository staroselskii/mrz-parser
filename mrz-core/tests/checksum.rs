use mrz_core::checksum::compute_checksum;

#[test]
fn test_checksum_numeric() {
    let input = b"123456789";
    assert_eq!(compute_checksum(input), Some(7));
}

#[test]
fn test_checksum_alphanumeric() {
    let input = b"A12B34C56";
    assert_eq!(compute_checksum(input), Some(0));
}

#[test]
fn test_checksum_fillers() {
    let input = b"<<<<<<<<<<<";
    assert_eq!(compute_checksum(input), Some(0));
}

#[test]
fn test_checksum_icao_td3_document_number() {
    // Example from ICAO 9303 TD3: L898902C3
    let input = b"L898902C3";
    assert_eq!(compute_checksum(input), Some(6));
}

#[test]
fn test_checksum_with_invalid_characters() {
    let input = b"L89*902C3";
    assert_eq!(compute_checksum(input), None); // * is invalid
}

#[test]
fn test_checksum_reference_data() {
    let cases: &[(&[u8], u8)] = &[
        (b"L898902C3", 6),                        // ICAO example document number
        (b"740812", 2),                           // ICAO example birth date
        (b"120415", 9),                           // ICAO example expiry date
        (b"ZE184226B", 1),                        // ICAO example optional data
        (b"L898902C37408121204159ZE184226B1", 0), // full composite field from ICAO spec
    ];

    for &(input, expected) in cases {
        assert_eq!(
            compute_checksum(input),
            Some(expected),
            "Checksum mismatch for input: {:?}",
            input
        );
    }
}
fn assert_checksum_matches(data: &[u8], expected_digit: u8) {
    assert_eq!(
        compute_checksum(data),
        Some(expected_digit),
        "Checksum failed for {:?}",
        std::str::from_utf8(data).unwrap()
    );
}

#[test]
fn test_field_level_checksums() {
    // Document number (TD3)
    assert_checksum_matches(b"L898902C3", 6);
    // Birth date
    assert_checksum_matches(b"740812", 2);
    // Expiry date
    assert_checksum_matches(b"120415", 9);
    // Optional data
    assert_checksum_matches(b"ZE184226B", 1);
    // Composite checksum over fields
    assert_checksum_matches(b"L898902C37408121204159ZE184226B1", 0);
}
