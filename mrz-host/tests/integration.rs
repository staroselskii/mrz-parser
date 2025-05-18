use mrz_core::MRZParseError;
use mrz_host::parse_lines;
use mrz_host::{MRZ, MRZBCBP, MRZICAO};
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Sample {
    format: String,
    lines: Vec<String>,
    document_number: Option<String>,
    name: Option<String>,
    birth_date: Option<String>,
    expiry_date: Option<String>,
    flight_number: Option<String>,
    seat: Option<String>,
}

#[test]
fn test_samples_from_fixtures() {
    let data = fs::read_to_string(Path::new("tests/fixtures/samples.json")).unwrap();
    let samples: Vec<Sample> = serde_json::from_str(&data).unwrap();

    for sample in samples {
        let line_refs: Vec<&str> = sample.lines.iter().map(String::as_str).collect();
        let parsed = parse_lines(&line_refs);

        match (sample.format.as_str(), parsed) {
            ("TD3", Ok(MRZ::ICAO(mrz))) => {
                if let Some(expected) = sample.document_number.as_deref() {
                    assert_eq!(mrz.document_number.trim_end_matches('<'), expected);
                }
                if let Some(expected) = sample.name.as_deref() {
                    assert_eq!(mrz.name.trim(), expected);
                }
                if let Some(expected) = sample.birth_date.as_deref() {
                    assert_eq!(mrz.birth_date.unwrap().to_string(), expected);
                }
                if let Some(expected) = sample.expiry_date.as_deref() {
                    assert_eq!(mrz.expiry_date.unwrap().to_string(), expected);
                }
            }
            ("BCBP", Ok(MRZ::BCBP(bcbp))) => {
                dbg!(&bcbp);
                dbg!(&sample);

                if let Some(expected) = sample.flight_number.as_deref() {
                    assert_eq!(bcbp.flight_number.trim_end_matches('<'), expected);
                }
                if let Some(expected) = sample.seat.as_deref() {
                    assert_eq!(bcbp.seat.trim_end_matches('<'), expected);
                }
            }
            _ => {
                dbg!(&sample.lines);
                panic!("Failed to parse expected format: {}", sample.format);
            }
        }
    }
}
