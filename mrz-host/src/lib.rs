use mrz_core::{parser::parse_any, MRZParseError, ParsedMRZ};
use time::{Date, Month};

#[derive(Debug)]
pub enum MRZ {
    IcaoTd3(MrzIcaoTd3),
    IcaoTd1(MrzIcaoTd1),
    Unknown,
}

#[derive(Debug)]
pub struct MrzIcaoTd3 {
    pub document_number: String,
    pub name: String,
    pub birth_date: Option<Date>,
    pub expiry_date: Option<Date>,
}

#[derive(Debug)]
pub struct MrzIcaoTd1 {
    pub document_code: String,
    pub issuing_state: String,
    pub name: String,
    pub document_number: String,
    pub nationality: String,
    pub birth_date: Option<Date>,
    pub birth_date_check: char,
    pub sex: char,
    pub expiry_date: Option<Date>,
    pub expiry_date_check: char,
    pub optional_data1: String,
    pub optional_data2: String,
    pub final_check: char,
}

fn normalize_lines(lines: &[&str]) -> Vec<Vec<u8>> {
    lines
        .iter()
        .map(|line| {
            let mut bytes = line.as_bytes().to_vec();
            let expected_len = match lines.len() {
                2 => 44, // TD3
                3 => 30, // TD1
                1 => 60, // BCBP (can vary)
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
    let normalized = normalize_lines(&lines);
    let refs: Vec<&[u8]> = normalized.iter().map(|l| &l[..]).collect();
    let parsed = parse_any(&refs)?;

    match parsed {
        ParsedMRZ::MrzIcaoTd3(raw) => Ok(MRZ::IcaoTd3(MrzIcaoTd3 {
            document_number: raw.document_number.to_string(),
            name: raw.name.to_string(),
            birth_date: parse_mrz_date(&raw.birth_date),
            expiry_date: parse_mrz_date(&raw.expiry_date),
        })),
        ParsedMRZ::MrzIcaoTd1(raw) => Ok(MRZ::IcaoTd1(MrzIcaoTd1 {
            document_code: parse_field(&raw.document_code),
            issuing_state: parse_field(&raw.issuing_state),
            name: raw.name.to_string(),
            document_number: raw.document_number.to_string(),
            nationality: parse_field(&raw.nationality),
            birth_date: parse_mrz_date(&raw.birth_date),
            birth_date_check: raw.birth_date_check as char,
            sex: raw.sex as char,
            expiry_date: parse_mrz_date(&raw.expiry_date),
            expiry_date_check: raw.expiry_date_check as char,
            optional_data1: raw.optional_data1.to_string(),
            optional_data2: raw.optional_data2.to_string(),
            final_check: raw.final_check.map(|b| b as char).unwrap_or('<'),
        })),
        ParsedMRZ::Unknown => Ok(MRZ::Unknown),
    }
}

fn parse_field(bytes: &[u8]) -> String {
    core::str::from_utf8(bytes)
        .unwrap_or("")
        .trim_end_matches('<')
        .to_string()
}

pub fn parse_mrz_date(raw: &[u8; 6]) -> Option<Date> {
    let s = core::str::from_utf8(raw).ok()?;
    let year = s[0..2].parse::<u16>().ok()?;
    let month = s[2..4].parse::<u8>().ok()?;
    let day = s[4..6].parse::<u8>().ok()?;

    let full_year = if year >= 50 { 1900 + year } else { 2000 + year };
    let month = Month::try_from(month).ok()?;
    Date::from_calendar_date(full_year as i32, month, day).ok()
}
