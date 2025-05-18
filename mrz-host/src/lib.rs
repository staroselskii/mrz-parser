use mrz_core::{parser::parse_any, MRZParseError, ParsedMRZ};
use time::{Date, Month};

#[derive(Debug)]
pub enum MRZ {
    ICAO(MRZICAO),
    BCBP(MRZBCBP),
    Unknown,
}

#[derive(Debug)]
pub struct MRZICAO {
    pub document_number: String,
    pub name: String,
    pub birth_date: Option<Date>,
    pub expiry_date: Option<Date>,
}

#[derive(Debug)]
pub struct MRZBCBP {
    pub passenger_name: String,
    pub flight_number: String,
    pub seat: String,
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
        ParsedMRZ::ICAO(raw) => Ok(MRZ::ICAO(MRZICAO {
            document_number: raw.document_number.to_string(),
            name: raw.name.to_string(),
            birth_date: parse_mrz_date(&raw.birth_date),
            expiry_date: parse_mrz_date(&raw.expiry_date),
        })),
        ParsedMRZ::BCBP(raw) => Ok(MRZ::BCBP(MRZBCBP {
            passenger_name: parse_field(&raw.passenger_name),
            flight_number: parse_field(&raw.flight_number),
            seat: parse_field(&raw.seat),
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
