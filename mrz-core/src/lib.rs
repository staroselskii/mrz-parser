#![cfg_attr(not(feature = "std"), no_std)]

extern crate heapless;
use heapless::String;

pub mod parser;

#[macro_export]
macro_rules! delegate_common_fields {
    ($field:ident) => {
        fn sex(&self) -> u8 {
            self.$field.sex
        }
        fn document_number(&self) -> &str {
            &self.$field.document_number
        }
        fn birth_date(&self) -> &[u8; ICAO_COMMON_DATE_LEN] {
            &self.$field.birth_date
        }
        fn expiry_date(&self) -> &[u8; ICAO_COMMON_DATE_LEN] {
            &self.$field.expiry_date
        }
        fn is_document_number_valid(&self) -> bool {
            self.$field.document_number_check_valid
        }
        fn is_birth_date_valid(&self) -> bool {
            self.$field.birth_date_check_valid
        }
        fn is_expiry_date_valid(&self) -> bool {
            self.$field.expiry_date_check_valid
        }
        fn is_final_check_valid(&self) -> Option<bool> {
            self.$field.final_check_valid
        }
    };
}

#[derive(Debug)]
struct MrzIcaoCommon {
    sex: u8,
    document_number: String<ICAO_COMMON_DOC_NUM_MAX_LEN>,
    document_number_check_valid: bool,
    birth_date: [u8; ICAO_COMMON_DATE_LEN],
    birth_date_check_valid: bool,
    expiry_date: [u8; ICAO_COMMON_DATE_LEN],
    expiry_date_check_valid: bool,
    final_check_valid: Option<bool>,
}

pub trait MrzIcaoCommonFields {
    fn sex(&self) -> u8;
    fn document_number(&self) -> &str;
    fn birth_date(&self) -> &[u8; ICAO_COMMON_DATE_LEN];
    fn expiry_date(&self) -> &[u8; ICAO_COMMON_DATE_LEN];
    fn is_document_number_valid(&self) -> bool;
    fn is_birth_date_valid(&self) -> bool;
    fn is_expiry_date_valid(&self) -> bool;
    fn is_final_check_valid(&self) -> Option<bool>;
}

#[derive(Debug)]
pub enum ParsedMRZ {
    MrzIcaoTd3(MrzIcaoTd3),
    MrzIcaoTd1(MrzIcaoTd1),
    Unknown,
}

pub const ICAO_COMMON_DATE_LEN: usize = 6;
pub const ICAO_COMMON_DOC_CODE_LEN: usize = 2;
pub const ICAO_COMMON_COUNTRY_CODE_LEN: usize = 3;
pub const ICAO_COMMON_DOC_NUM_MAX_LEN: usize = 9;

pub const ICAO_TD3_NAME_MAX_LEN: usize = 39;
pub const ICAO_TD3_OPTIONAL1_MAX_LEN: usize = 15;
pub const ICAO_TD3_OPTIONAL2_MAX_LEN: usize = 11;

#[derive(Debug)]
pub struct MrzIcaoTd3 {
    pub document_code: [u8; ICAO_COMMON_DOC_CODE_LEN],
    pub issuing_state: [u8; ICAO_COMMON_COUNTRY_CODE_LEN],
    pub name: String<ICAO_TD3_NAME_MAX_LEN>,
    pub nationality: [u8; ICAO_COMMON_COUNTRY_CODE_LEN],
    pub optional_data1: String<ICAO_TD3_OPTIONAL1_MAX_LEN>,
    pub optional_data2: String<ICAO_TD3_OPTIONAL2_MAX_LEN>,
    common: MrzIcaoCommon,
}

impl MrzIcaoCommonFields for MrzIcaoTd3 {
    delegate_common_fields!(common);
}

pub const ICAO_TD1_NAME_MAX_LEN: usize = 30;
pub const ICAO_TD1_OPTIONAL1_MAX_LEN: usize = 15;
pub const ICAO_TD1_OPTIONAL2_MAX_LEN: usize = 11;

#[derive(Debug)]
pub struct MrzIcaoTd1 {
    pub document_code: [u8; ICAO_COMMON_DOC_CODE_LEN],
    pub issuing_state: [u8; ICAO_COMMON_COUNTRY_CODE_LEN],
    pub name: String<ICAO_TD1_NAME_MAX_LEN>,
    pub nationality: [u8; ICAO_COMMON_COUNTRY_CODE_LEN],
    pub optional_data1: String<ICAO_TD1_OPTIONAL1_MAX_LEN>,
    pub optional_data2: String<ICAO_TD1_OPTIONAL2_MAX_LEN>,
    common: MrzIcaoCommon,
}

impl MrzIcaoCommonFields for MrzIcaoTd1 {
    delegate_common_fields!(common);
}

#[derive(Debug, PartialEq, Eq)]
pub enum MRZFormat {
    MrzIcaoTd3,
    MrzIcaoTd1,
    BCBP,
    Unknown,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MRZParseError {
    InvalidLength,
    InvalidChecksumField(MRZChecksumError),
    UnknownFormat,
    UnsupportedFormat,
    Utf8Error,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MRZChecksumError {
    DocumentNumber,
    BirthDate,
    ExpiryDate,
    Final,
}
