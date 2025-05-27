use crate::date::parse_mrz_date_with_reference;
use crate::validation::validate_common_fields;
use mrz_core::MrzIcaoCommonFields;

use crate::MRZ;
use mrz_core::{parser::parse_any, MRZParseError, ParsedMRZ};

fn normalize_lines(lines: &[&str]) -> Vec<Vec<u8>> {
    lines
        .iter()
        .map(|line| {
            let mut bytes = line.as_bytes().to_vec();
            let expected_len = match lines.len() {
                2 => 44,
                3 => 30,
                1 => 60,
                _ => 0,
            };
            while bytes.len() < expected_len {
                bytes.push(b'<');
            }
            bytes
        })
        .collect()
}

fn build_mrz_result<T: MrzIcaoCommonFields>(
    raw: &T,
    format: &str,
) -> Result<MRZ, MRZParseError> {
    let birth_date_bytes = raw.birth_date();
    let expiry_date_bytes = raw.expiry_date();
    Ok(MRZ::Icao(crate::model::MrzIcaoUnified::from_common_fields(
        raw,
        format,
        raw.surname().as_str(),
        raw.given_names().as_str(),
        parse_mrz_date_with_reference(birth_date_bytes, Some(expiry_date_bytes)),
        parse_mrz_date_with_reference(expiry_date_bytes, None),
        raw.sex() as char,
    )))
}

pub fn parse_lines(lines: &[&str]) -> Result<MRZ, MRZParseError> {
    let normalized = normalize_lines(lines);
    let refs: Vec<&[u8]> = normalized.iter().map(|l| &l[..]).collect();
    let parsed = parse_any(&refs)?;

    match parsed {
        ParsedMRZ::MrzIcaoTd3(ref raw) => {
            validate_common_fields(raw)?;
            build_mrz_result(raw, "TD3")
        }
        ParsedMRZ::MrzIcaoTd1(ref raw) => {
            validate_common_fields(raw)?;
            build_mrz_result(raw, "TD1")
        }
        ParsedMRZ::Unknown => Ok(MRZ::Unknown),
    }
}
