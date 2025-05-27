//! MRZ (Machine Readable Zone) parsing core library.
//!
//! This crate provides core data structures and utilities for parsing ICAO-compliant MRZ formats,
//! including TD1 and TD3. It is designed to be `no_std` compatible and safe for embedded and MCU use.
#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

use heapless::String;
use heapless::Vec;

/// MRZ checksum validation utilities.
pub mod checksum;
/// MRZ format parsing utilities and functions.
pub mod parser;

/// Splits the MRZ name field into surname and given names.
///
/// The MRZ name field uses `<<` as a delimiter between surname and given names,
/// and `<` as a space character within names.
///
/// # Arguments
///
/// * `name_field` - The raw MRZ name field string.
///
/// # Returns
///
/// A tuple containing the surname and given names as separate `String`s.
fn split_name_field<const N: usize>(name_field: &str) -> (String<N>, String<N>) {
    let mut surname = String::<N>::new();
    let mut given_names = String::<N>::new();

    let mut parts = name_field.splitn(2, "<<");
    if let Some(s) = parts.next() {
        let _ = surname.push_str(s);
    }

    if let Some(given_raw) = parts.next() {
        for c in given_raw.chars() {
            let ch = if c == '<' { ' ' } else { c };
            if given_names.push(ch).is_err() {
                break;
            }
        }
    }

    (surname, given_names)
}

/// Common field interface shared across ICAO MRZ formats (TD1, TD3), implemented for `MrzIcao<...>`.
pub trait MrzIcaoCommonFields {
    /// Returns the sex field value as a single ASCII byte (e.g., 'M', 'F', or '<').
    fn sex(&self) -> u8;
    /// Returns the document number as a string slice.
    fn document_number(&self) -> &str;
    /// Returns the birth date as a byte array (YYMMDD).
    fn birth_date(&self) -> &[u8; ICAO_COMMON_DATE_LEN];
    /// Returns the expiry date as a byte array (YYMMDD).
    fn expiry_date(&self) -> &[u8; ICAO_COMMON_DATE_LEN];
    /// Returns whether the document number passed checksum validation.
    fn is_document_number_valid(&self) -> bool;
    /// Returns whether the birth date passed checksum validation.
    fn is_birth_date_valid(&self) -> bool;
    /// Returns whether the expiry date passed checksum validation.
    fn is_expiry_date_valid(&self) -> bool;
    /// Returns whether the final checksum passed validation, if applicable.
    fn is_final_check_valid(&self) -> Option<bool>;
    /// Returns the surname parsed from the name field.
    fn surname(&self) -> String<ICAO_TD3_NAME_MAX_LEN>;
    /// Returns the given names parsed from the name field.
    fn given_names(&self) -> String<ICAO_TD3_NAME_MAX_LEN>;
    /// Returns the issuing state or organization as a 3-character country code.
    fn issuing_state(&self) -> &[u8; ICAO_COMMON_COUNTRY_CODE_LEN];
    /// Returns the nationality as a 3-character country code.
    fn nationality(&self) -> &[u8; ICAO_COMMON_COUNTRY_CODE_LEN];

    /// Returns the document code (e.g., "P<" for passport).
    fn document_code(&self) -> &[u8; ICAO_COMMON_DOC_CODE_LEN];

    /// Returns the raw name field (surname and given names joined by '<<').
    fn raw_name(&self) -> &str;

    /// Returns whether the final check digit is required and present.
    fn has_final_check(&self) -> bool;

    /// Returns optional data field 1.
    fn optional_data1(&self) -> &str;

    /// Returns optional data field 2.
    fn optional_data2(&self) -> &str;
}

/// Parsed MRZ format variants.
pub enum ParsedMRZ {
    /// TD3 format.
    MrzIcaoTd3(MrzIcaoTd3),
    /// TD1 format.
    MrzIcaoTd1(MrzIcaoTd1),
    /// Unknown or unsupported format.
    Unknown,
}

#[cfg_attr(not(feature = "std"), derive(Debug))]
impl core::fmt::Debug for ParsedMRZ {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ParsedMRZ::MrzIcaoTd3(td3) => f.debug_tuple("MrzIcaoTd3").field(td3).finish(),
            ParsedMRZ::MrzIcaoTd1(td1) => f.debug_tuple("MrzIcaoTd1").field(td1).finish(),
            ParsedMRZ::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Length of date fields (YYMMDD) in ICAO MRZ formats.
pub const ICAO_COMMON_DATE_LEN: usize = 6;
/// Length of document code field in ICAO MRZ formats.
pub const ICAO_COMMON_DOC_CODE_LEN: usize = 2;
/// Length of country code fields in ICAO MRZ formats.
pub const ICAO_COMMON_COUNTRY_CODE_LEN: usize = 3;
/// Maximum length of document number field in ICAO MRZ formats.
pub const ICAO_COMMON_DOC_NUM_MAX_LEN: usize = 9;

/// Maximum length of name field in ICAO TD3 format.
pub const ICAO_TD3_NAME_MAX_LEN: usize = 39;
/// Maximum length of optional data field 1 in ICAO TD3 format.
pub const ICAO_TD3_OPTIONAL1_MAX_LEN: usize = 15;
/// Maximum length of optional data field 2 in ICAO TD3 format.
pub const ICAO_TD3_OPTIONAL2_MAX_LEN: usize = 11;

/// Generic ICAO document representation parameterized by length constants.
pub struct MrzIcao<const NAME_LEN: usize, const OPT1_LEN: usize, const OPT2_LEN: usize> {
    /// Document code (e.g., "P<" for passport).
    pub document_code: [u8; ICAO_COMMON_DOC_CODE_LEN],
    /// Issuing country or organization code (3-letter).
    pub issuing_state: [u8; ICAO_COMMON_COUNTRY_CODE_LEN],
    /// Raw MRZ name field (surname and given names separated by `<<`).
    pub name: String<NAME_LEN>,
    /// Nationality country code (3-letter).
    pub nationality: [u8; ICAO_COMMON_COUNTRY_CODE_LEN],
    /// Sex character ('M', 'F', or '<').
    pub sex: u8,
    /// Document number.
    pub document_number: String<ICAO_COMMON_DOC_NUM_MAX_LEN>,
    /// Whether the document number passed checksum validation.
    pub document_number_check_valid: bool,
    /// Date of birth (YYMMDD).
    pub birth_date: [u8; ICAO_COMMON_DATE_LEN],
    /// Whether the birth date passed checksum validation.
    pub birth_date_check_valid: bool,
    /// Expiry date (YYMMDD).
    pub expiry_date: [u8; ICAO_COMMON_DATE_LEN],
    /// Whether the expiry date passed checksum validation.
    pub expiry_date_check_valid: bool,
    /// Whether the final check digit passed validation, if applicable.
    pub final_check_valid: Option<bool>,
    /// First optional data field.
    pub optional_data1: String<OPT1_LEN>,
    /// Second optional data field.
    pub optional_data2: String<OPT2_LEN>,
}

/// ICAO MRZ TD3 document type (e.g., passport), with fixed field lengths.
pub type MrzIcaoTd3 =
    MrzIcao<ICAO_TD3_NAME_MAX_LEN, ICAO_TD3_OPTIONAL1_MAX_LEN, ICAO_TD3_OPTIONAL2_MAX_LEN>;

#[cfg_attr(not(feature = "std"), derive(Debug))]
impl<const NAME_LEN: usize, const OPT1_LEN: usize, const OPT2_LEN: usize> core::fmt::Debug
    for MrzIcao<NAME_LEN, OPT1_LEN, OPT2_LEN>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let birth = core::str::from_utf8(&self.birth_date).unwrap_or("??");
        let expiry = core::str::from_utf8(&self.expiry_date).unwrap_or("??");

        f.debug_struct("MrzIcao")
            .field("document_code", &self.document_code)
            .field("issuing_state", &self.issuing_state)
            .field("name", &self.name)
            .field("surname", &self.surname())
            .field("given_names", &self.given_names())
            .field("nationality", &self.nationality)
            .field("sex", &self.sex)
            .field("document_number", &self.document_number)
            .field(
                "document_number_check_valid",
                &self.document_number_check_valid,
            )
            .field("birth_date", &birth)
            .field("birth_date_check_valid", &self.birth_date_check_valid)
            .field("expiry_date", &expiry)
            .field("expiry_date_check_valid", &self.expiry_date_check_valid)
            .field("final_check_valid", &self.final_check_valid)
            .field("optional_data1", &self.optional_data1)
            .field("optional_data2", &self.optional_data2)
            .finish()
    }
}

impl<const NAME_LEN: usize, const OPT1_LEN: usize, const OPT2_LEN: usize> MrzIcaoCommonFields
    for MrzIcao<NAME_LEN, OPT1_LEN, OPT2_LEN>
{
    fn sex(&self) -> u8 {
        self.sex
    }
    fn document_number(&self) -> &str {
        &self.document_number
    }
    fn birth_date(&self) -> &[u8; ICAO_COMMON_DATE_LEN] {
        &self.birth_date
    }
    fn expiry_date(&self) -> &[u8; ICAO_COMMON_DATE_LEN] {
        &self.expiry_date
    }
    fn is_document_number_valid(&self) -> bool {
        self.document_number_check_valid
    }
    fn is_birth_date_valid(&self) -> bool {
        self.birth_date_check_valid
    }
    fn is_expiry_date_valid(&self) -> bool {
        self.expiry_date_check_valid
    }
    fn is_final_check_valid(&self) -> Option<bool> {
        self.final_check_valid
    }

    fn surname(&self) -> String<ICAO_TD3_NAME_MAX_LEN> {
        split_name_field(&self.name).0
    }
    fn given_names(&self) -> String<ICAO_TD3_NAME_MAX_LEN> {
        split_name_field(&self.name).1
    }
    fn issuing_state(&self) -> &[u8; ICAO_COMMON_COUNTRY_CODE_LEN] {
        &self.issuing_state
    }
    fn nationality(&self) -> &[u8; ICAO_COMMON_COUNTRY_CODE_LEN] {
        &self.nationality
    }

    fn document_code(&self) -> &[u8; ICAO_COMMON_DOC_CODE_LEN] {
        &self.document_code
    }

    fn raw_name(&self) -> &str {
        &self.name
    }

    fn has_final_check(&self) -> bool {
        self.final_check_valid.is_some()
    }

    fn optional_data1(&self) -> &str {
        &self.optional_data1
    }

    fn optional_data2(&self) -> &str {
        &self.optional_data2
    }
}

/// Maximum length of name field in ICAO TD1 format.
pub const ICAO_TD1_NAME_MAX_LEN: usize = 30;
/// Maximum length of optional data field 1 in ICAO TD1 format.
pub const ICAO_TD1_OPTIONAL1_MAX_LEN: usize = 15;
/// Maximum length of optional data field 2 in ICAO TD1 format.
pub const ICAO_TD1_OPTIONAL2_MAX_LEN: usize = 11;

/// ICAO MRZ TD1 document type (e.g., ID card), with fixed field lengths.
pub type MrzIcaoTd1 =
    MrzIcao<ICAO_TD1_NAME_MAX_LEN, ICAO_TD1_OPTIONAL1_MAX_LEN, ICAO_TD1_OPTIONAL2_MAX_LEN>;

/// MRZ document format types.
#[derive(Debug, PartialEq, Eq)]
pub enum MRZFormat {
    /// TD3 format (passport).
    MrzIcaoTd3,
    /// TD1 format (ID card).
    MrzIcaoTd1,
    /// BCBP format (boarding pass).
    BCBP,
    /// Unknown or unsupported format.
    Unknown,
}

/// MRZ parsing error types.
#[derive(Debug, PartialEq, Eq)]
pub enum MRZParseError {
    /// Input length is invalid for any known MRZ format.
    InvalidLength,
    /// Checksum validation failed for a specific field.
    InvalidChecksumField(MRZChecksumError),
    /// MRZ format could not be determined.
    UnknownFormat,
    /// MRZ format is recognized but not supported.
    UnsupportedFormat,
    /// UTF-8 decoding error occurred.
    Utf8Error,
}

/// MRZ checksum validation error types.
#[derive(Debug, PartialEq, Eq)]
pub enum MRZChecksumError {
    /// Document number checksum failed.
    DocumentNumber,
    /// Birth date checksum failed.
    BirthDate,
    /// Expiry date checksum failed.
    ExpiryDate,
    /// Final composite checksum failed.
    Final,
}

#[cfg(feature = "std")]
impl core::fmt::Display for MRZChecksumError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MRZChecksumError::DocumentNumber => f.write_str("DocumentNumber"),
            MRZChecksumError::BirthDate => f.write_str("BirthDate"),
            MRZChecksumError::ExpiryDate => f.write_str("ExpiryDate"),
            MRZChecksumError::Final => f.write_str("Final"),
        }
    }
}

#[cfg(not(feature = "std"))]
impl core::fmt::Display for MRZChecksumError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MRZChecksumError::DocumentNumber => f.write_str("DocumentNumber"),
            MRZChecksumError::BirthDate => f.write_str("BirthDate"),
            MRZChecksumError::ExpiryDate => f.write_str("ExpiryDate"),
            MRZChecksumError::Final => f.write_str("Final"),
        }
    }
}

/// Detects the MRZ format of the given input string.
///
/// # Arguments
///
/// * `input` - A string slice representing the raw MRZ input.
///
/// # Returns
///
/// An `MRZFormat` enum variant indicating the detected MRZ format.
pub fn detect_format(input: &str) -> MRZFormat {
    let mut lines: Vec<&[u8], 3> = Vec::new();
    for line in input.lines() {
        let _ = lines.push(line.as_bytes());
    }
    parser::detect_format(&lines)
}

/// Parses the given MRZ input string into a parsed MRZ structure.
///
/// # Arguments
///
/// * `input` - A string slice representing the raw MRZ input.
///
/// # Returns
///
/// A `Result` containing a `ParsedMRZ` enum variant on success,
/// or an `MRZParseError` on failure.
pub fn parse_any(input: &str) -> Result<ParsedMRZ, MRZParseError> {
    let mut lines: Vec<&[u8], 3> = Vec::new();
    for line in input.lines() {
        let _ = lines.push(line.as_bytes());
    }
    parser::parse_any(&lines)
}
