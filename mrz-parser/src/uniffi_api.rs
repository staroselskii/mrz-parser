use mrz_host::{parse_lines as inner_parse_lines, MRZ};

use uniffi::export;

#[derive(uniffi::Record)]
pub struct MrzResult {
    pub document_type: String,
    pub document_number: String,
    pub name: String,
    pub nationality: String,
    pub birth_date: String,
    pub sex: String,
    pub expiry_date: String,
    pub optional_data1: String,
    pub optional_data2: String,
    pub issuing_state: String,
    pub given_names: String,
    pub surname: String,
}

use thiserror::Error;

#[derive(uniffi::Error, Debug, Error)]
pub enum MrzParseError {
    #[error("The MRZ has an invalid length")]
    InvalidLength,
    #[error("Checksum failed: {0}")]
    InvalidChecksumField(String),
    #[error("Unknown format")]
    UnknownFormat,
    #[error("Unsupported format")]
    UnsupportedFormat,
    #[error("UTF-8 error")]
    Utf8Error,
}

#[export]
pub fn parse_lines(lines: Vec<String>) -> Result<MrzResult, MrzParseError> {
    let strs: Vec<&str> = lines.iter().map(|s| s.as_str()).collect();
    inner_parse_lines(&strs)
        .map_err(|e| match e {
            mrz_core::MRZParseError::InvalidLength => MrzParseError::InvalidLength,
            mrz_core::MRZParseError::InvalidChecksumField(inner) => {
                MrzParseError::InvalidChecksumField(format!("{:?}", inner))
            }
            mrz_core::MRZParseError::UnknownFormat => MrzParseError::UnknownFormat,
            mrz_core::MRZParseError::UnsupportedFormat => MrzParseError::UnsupportedFormat,
            mrz_core::MRZParseError::Utf8Error => MrzParseError::Utf8Error,
        })
        .and_then(|mrz| match mrz {
            MRZ::Icao(u) => Ok(MrzResult {
                document_type: u.document_code().to_string(),
                document_number: u.document_number().to_string(),
                name: u.full_name().to_string(),
                nationality: u.nationality().to_string(),
                birth_date: u.birth_date().map_or("".into(), |d| d.to_string()),
                sex: u.sex().to_string(),
                expiry_date: u.expiry_date().map_or("".into(), |d| d.to_string()),
                optional_data1: u.optional_data1().to_string(),
                optional_data2: u.optional_data2().to_string(),
                issuing_state: u.issuing_state().to_string(),
                given_names: u.given_names().to_string(),
                surname: u.surname().to_string(),
            }),
            MRZ::Unknown => Err(MrzParseError::UnknownFormat),
        })
}
