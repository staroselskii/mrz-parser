//! # MRZ Parser
//! A compliance-ready library for parsing MRZs (Machine Readable Zones).

#![deny(missing_docs)]

use std::default::*;

/// Parsing options for MRZ parsing.
#[derive(Default, Debug, Clone, Copy)]
pub struct ParseOptions {
    /// Whether to strictly validate the final overall checksum.
    pub validate_final_checksum: bool,
}

/// Errors that can occur during MRZ parsing.
#[derive(Debug)]
pub enum MRZParseError {
    /// The MRZ input was empty or invalid.
    EmptyInput,
    /// The MRZ must have exactly two lines of 44 characters.
    InvalidFormat,
    /// Specific field parsing failed.
    FieldError(&'static str),
}

/// Represents parsed MRZ data.
#[derive(Debug)]
pub struct MRZData {
    /// Document type (e.g., P for Passport).
    pub document_type: String,
    /// Issuing country code (ISO-3166-1 alpha-3).
    pub issuing_country: String,
    /// Tuple of (surname, given names).
    pub names: (String, String),
    /// Passport number field.
    pub passport_number: String,
    /// Nationality country code.
    pub nationality: String,
    /// Date of birth (YYMMDD format).
    pub birth_date: String,
    /// Sex (M, F, or < for unspecified).
    pub sex: String,
    /// Expiration date of the document (YYMMDD format).
    pub expiry_date: String,
}

pub mod parser;
pub mod validation;

use std::str::FromStr;

impl FromStr for MRZData {
    type Err = MRZParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().map(str::trim_end).collect();
        parse_mrz(&lines)
    }
}

/// Parses MRZ input using default options.
pub fn parse_mrz(lines: &[&str]) -> Result<MRZData, MRZParseError> {
    parse_mrz_with_options(lines, ParseOptions::default())
}

/// Parses MRZ input with custom options.
pub fn parse_mrz_with_options(
    lines: &[&str],
    options: ParseOptions,
) -> Result<MRZData, MRZParseError> {
    match (lines.len(), lines.first().map(|l| l.len())) {
        (2, Some(44)) => parser::td3::parse_td3_mrz_strict(lines, options),
        _ => Err(MRZParseError::InvalidFormat),
    }
}
