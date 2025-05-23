use crate::date::parse_mrz_date_with_reference;
use crate::model::{MrzIcaoUnified, MRZ};
use crate::util::{parse_field, parse_str_field};
use crate::validation::{validate_common_fields, validate_td1_fields};
use mrz_core::MrzIcaoCommonFields;

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

pub fn parse_lines(lines: &[&str]) -> Result<MRZ, MRZParseError> {
    let normalized = normalize_lines(lines);
    let refs: Vec<&[u8]> = normalized.iter().map(|l| &l[..]).collect();
    let parsed = parse_any(&refs)?;

    match parsed {
        ParsedMRZ::MrzIcaoTd3(raw) => {
            validate_common_fields(&raw)?;
            let birth_date_bytes = raw.birth_date();
            let expiry_date_bytes = raw.expiry_date();
            Ok(MRZ::Icao(crate::model::MrzIcaoUnified {
                document_number: parse_str_field(raw.document_number()),
                name: raw.name.to_string(),
                birth_date: parse_mrz_date_with_reference(
                    birth_date_bytes,
                    Some(expiry_date_bytes),
                ),
                expiry_date: parse_mrz_date_with_reference(expiry_date_bytes, None),
                sex: raw.sex() as char,
                optional_data1: raw.optional_data1.to_string(),
                optional_data2: raw.optional_data2.to_string(),
                final_check: raw.is_final_check_valid(),
                nationality: parse_field(&raw.nationality),
                issuing_state: parse_field(&raw.issuing_state),
                document_code: parse_field(&raw.document_code),
                format: "TD3".to_string(),
            }))
        }
        ParsedMRZ::MrzIcaoTd1(raw) => {
            validate_td1_fields(&raw)?;
            let birth_date_bytes = raw.birth_date();
            let expiry_date_bytes = raw.expiry_date();
            Ok(MRZ::Icao(crate::model::MrzIcaoUnified {
                document_number: parse_str_field(raw.document_number()),
                name: raw.name.to_string(),
                birth_date: parse_mrz_date_with_reference(
                    birth_date_bytes,
                    Some(expiry_date_bytes),
                ),
                expiry_date: parse_mrz_date_with_reference(expiry_date_bytes, None),
                sex: raw.sex() as char,
                optional_data1: raw.optional_data1.to_string(),
                optional_data2: raw.optional_data2.to_string(),
                final_check: raw.is_final_check_valid(),
                nationality: parse_field(&raw.nationality),
                issuing_state: parse_field(&raw.issuing_state),
                document_code: parse_field(&raw.document_code),
                format: "TD1".to_string(),
            }))
        }
        ParsedMRZ::Unknown => Ok(MRZ::Unknown),
    }
}
