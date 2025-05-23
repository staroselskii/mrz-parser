use mrz_core::{
    parser::parse_any, MRZChecksumError, MRZParseError, MrzIcaoCommonFields, ParsedMRZ,
};
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
    pub sex: char,
    pub optional_data1: String,
    pub optional_data2: String,
    pub final_check: Option<bool>,
}

#[derive(Debug)]
pub struct MrzIcaoTd1 {
    pub document_code: String,
    pub issuing_state: String,
    pub name: String,
    pub document_number: String,
    pub nationality: String,
    pub birth_date: Option<Date>,
    pub birth_date_check: bool,
    pub sex: char,
    pub expiry_date: Option<Date>,
    pub expiry_date_check: bool,
    pub optional_data1: String,
    pub optional_data2: String,
    pub final_check: Option<bool>,
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
        ParsedMRZ::MrzIcaoTd3(raw) => {
            if !raw.is_birth_date_valid() {
                return Err(MRZParseError::InvalidChecksumField(
                    MRZChecksumError::BirthDate,
                ));
            }
            if !raw.is_expiry_date_valid() {
                return Err(MRZParseError::InvalidChecksumField(
                    MRZChecksumError::ExpiryDate,
                ));
            }
            if raw.is_final_check_valid() == Some(false) {
                return Err(MRZParseError::InvalidChecksumField(MRZChecksumError::Final));
            }
            let birth_date_bytes = raw.birth_date();
            let expiry_date_bytes = raw.expiry_date();
            Ok(MRZ::IcaoTd3(MrzIcaoTd3 {
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
            }))
        }
        ParsedMRZ::MrzIcaoTd1(raw) => {
            if !raw.is_birth_date_valid() {
                return Err(MRZParseError::InvalidChecksumField(
                    MRZChecksumError::BirthDate,
                ));
            }
            if !raw.is_expiry_date_valid() {
                return Err(MRZParseError::InvalidChecksumField(
                    MRZChecksumError::ExpiryDate,
                ));
            }
            if !raw.is_document_number_valid() {
                return Err(MRZParseError::InvalidChecksumField(
                    MRZChecksumError::DocumentNumber,
                ));
            }
            if raw.is_final_check_valid() == Some(false) {
                return Err(MRZParseError::InvalidChecksumField(MRZChecksumError::Final));
            }
            let birth_date_bytes = raw.birth_date();
            let expiry_date_bytes = raw.expiry_date();
            Ok(MRZ::IcaoTd1(MrzIcaoTd1 {
                document_code: parse_field(&raw.document_code),
                issuing_state: parse_field(&raw.issuing_state),
                name: raw.name.to_string(),
                document_number: parse_str_field(raw.document_number()),
                nationality: parse_field(&raw.nationality),
                birth_date: parse_mrz_date_with_reference(
                    birth_date_bytes,
                    Some(expiry_date_bytes),
                ),
                birth_date_check: raw.is_birth_date_valid(),
                sex: raw.sex() as char,
                expiry_date: parse_mrz_date_with_reference(expiry_date_bytes, None),
                expiry_date_check: raw.is_expiry_date_valid(),
                optional_data1: raw.optional_data1.to_string(),
                optional_data2: raw.optional_data2.to_string(),
                final_check: raw.is_final_check_valid(),
            }))
        }
        ParsedMRZ::Unknown => Ok(MRZ::Unknown),
    }
}

fn parse_field(bytes: &[u8]) -> String {
    core::str::from_utf8(bytes)
        .unwrap_or("")
        .trim_end_matches('<')
        .to_string()
}

fn parse_str_field(s: &str) -> String {
    s.trim_end_matches('<').to_string()
}

pub fn parse_mrz_date_with_reference(date: &[u8; 6], reference: Option<&[u8; 6]>) -> Option<Date> {
    let year = core::str::from_utf8(&date[0..2])
        .ok()?
        .parse::<u16>()
        .ok()?;
    let month = core::str::from_utf8(&date[2..4]).ok()?.parse::<u8>().ok()?;
    let day = core::str::from_utf8(&date[4..6]).ok()?.parse::<u8>().ok()?;

    let full_year = if let Some(ref_date) = reference {
        let ref_year = core::str::from_utf8(&ref_date[0..2])
            .ok()?
            .parse::<u16>()
            .ok()?;
        let ref_century = if ref_year >= 50 { 1900 } else { 2000 };
        let ref_full_year = ref_century + ref_year;

        let candidate = 1900 + year;
        if candidate > ref_full_year {
            2000 + year
        } else {
            candidate
        }
    } else {
        if year >= 50 {
            1900 + year
        } else {
            2000 + year
        }
    };

    let month = Month::try_from(month).ok()?;
    Date::from_calendar_date(full_year as i32, month, day).ok()
}
