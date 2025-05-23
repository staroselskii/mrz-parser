use mrz_core::MRZParseError;
use mrz_host::parse_lines;
use mrz_host::MRZ;
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
    sex: Option<String>,
    optional_data1: Option<String>,
    optional_data2: Option<String>,
    invalid_checksums: Option<Vec<String>>,
}

#[test]
fn test_samples_valid() {
    let data = fs::read_to_string(Path::new("tests/fixtures/samples.json")).unwrap();
    let samples: Vec<Sample> = serde_json::from_str(&data).unwrap();

    for sample in samples.iter().filter(|s| s.invalid_checksums.is_none()) {
        let line_refs: Vec<&str> = sample.lines.iter().map(String::as_str).collect();
        let parsed = parse_lines(&line_refs);

        match (sample.format.as_str(), parsed) {
            ("TD1", Ok(MRZ::Icao(mrz))) => {
                if let Some(expected) = sample.document_number.as_deref() {
                    assert_eq!(mrz.document_number.trim_end_matches('<'), expected);
                }
                if let Some(expected) = sample.name.as_deref() {
                    assert_eq!(mrz.name.trim(), expected);
                }
                if let Some(expected) = sample.expiry_date.as_deref() {
                    assert_eq!(mrz.expiry_date.unwrap().to_string(), expected);
                }
                if let Some(expected) = sample.birth_date.as_deref() {
                    assert_eq!(mrz.birth_date.unwrap().to_string(), expected);
                }
                if let Some(expected) = sample.sex.as_deref() {
                    assert_eq!(mrz.sex.to_string(), expected);
                }
                if let Some(expected) = sample.optional_data1.as_deref() {
                    assert_eq!(mrz.optional_data1.trim_end_matches('<'), expected);
                }
                if let Some(expected) = sample.optional_data2.as_deref() {
                    assert_eq!(mrz.optional_data2.trim_end_matches('<'), expected);
                }
            }
            ("TD3", Ok(MRZ::Icao(mrz))) => {
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
                if let Some(expected) = sample.sex.as_deref() {
                    assert_eq!(mrz.sex.to_string(), expected);
                }
                if let Some(expected) = sample.optional_data1.as_deref() {
                    assert_eq!(mrz.optional_data1.trim_end_matches('<'), expected);
                }
                if let Some(expected) = sample.optional_data2.as_deref() {
                    assert_eq!(mrz.optional_data2.trim_end_matches('<'), expected);
                }
            }
            (_, Err(e)) => {
                dbg!(&sample.lines);
                dbg!(e);
                panic!("Failed to parse expected format: {}", sample.format);
            }
            _ => {
                dbg!(&sample.lines);
                panic!("Unexpected format: {}", sample.format);
            }
        }
    }
}

#[test]
fn test_samples_invalid() {
    let data = fs::read_to_string(Path::new("tests/fixtures/samples.json")).unwrap();
    let samples: Vec<Sample> = serde_json::from_str(&data).unwrap();

    for sample in samples.into_iter().filter(|s| {
        s.invalid_checksums
            .as_ref()
            .map_or(false, |v| !v.is_empty())
    }) {
        let line_refs: Vec<&str> = sample.lines.iter().map(String::as_str).collect();
        let parsed = parse_lines(&line_refs);

        match parsed {
            Err(MRZParseError::InvalidChecksumField(err)) => {
                let mut err_strings: Vec<String> = vec![err.to_string()];
                err_strings.sort();
                let mut expected = sample.invalid_checksums.unwrap();
                expected.sort();
                assert_eq!(err_strings, expected);
            }
            _ => {
                dbg!(&sample.lines);
                panic!("Expected checksum error for format: {}", sample.format);
            }
        }
    }
}
