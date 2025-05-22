#![cfg_attr(not(feature = "std"), no_std)]

extern crate heapless;
use heapless::String;

pub mod parser;

#[derive(Debug)]
pub enum ParsedMRZ {
    MrzIcaoTd3(MrzIcaoTd3),
    MrzIcaoTd1(MrzIcaoTd1),
    Unknown,
}

pub const ICAO_COMMON_DATE_LEN: usize = 6;
pub const ICAO_COMMON_DOC_CODE_LEN: usize = 2;
pub const ICAO_COMMON_COUNTRY_CODE_LEN: usize = 3;

pub const ICAO_TD3_DOC_NUM_MAX_LEN: usize = 9;
pub const ICAO_TD3_NAME_MAX_LEN: usize = 39;

#[derive(Debug)]
pub struct MrzIcaoTd3 {
    pub document_number: String<ICAO_TD3_DOC_NUM_MAX_LEN>,
    pub document_number_check: u8,
    pub document_number_check_valid: bool,
    pub name: String<ICAO_TD3_NAME_MAX_LEN>,
    pub birth_date: [u8; ICAO_COMMON_DATE_LEN],
    pub birth_date_check: u8,
    pub birth_date_check_valid: bool,
    pub expiry_date: [u8; ICAO_COMMON_DATE_LEN],
    pub expiry_date_check: u8,
    pub expiry_date_check_valid: bool,
    pub final_check: Option<u8>,
    pub final_check_valid: Option<bool>,
}

pub const ICAO_TD1_DOC_NUM_MAX_LEN: usize = 9;
pub const ICAO_TD1_NAME_MAX_LEN: usize = 30;
pub const ICAO_TD1_OPTIONAL1_MAX_LEN: usize = 15;
pub const ICAO_TD1_OPTIONAL2_MAX_LEN: usize = 11;

#[derive(Debug)]
pub struct MrzIcaoTd1 {
    pub document_code: [u8; ICAO_COMMON_DOC_CODE_LEN],
    pub issuing_state: [u8; ICAO_COMMON_COUNTRY_CODE_LEN],
    pub name: String<ICAO_TD1_NAME_MAX_LEN>,
    pub document_number: String<ICAO_TD1_DOC_NUM_MAX_LEN>,
    pub document_number_check: u8,
    pub document_number_check_valid: bool,
    pub nationality: [u8; ICAO_COMMON_COUNTRY_CODE_LEN],
    pub birth_date: [u8; ICAO_COMMON_DATE_LEN],
    pub birth_date_check: u8,
    pub birth_date_check_valid: bool,
    pub sex: u8,
    pub expiry_date: [u8; ICAO_COMMON_DATE_LEN],
    pub expiry_date_check: u8,
    pub expiry_date_check_valid: bool,
    pub optional_data1: String<ICAO_TD1_OPTIONAL1_MAX_LEN>,
    pub optional_data2: String<ICAO_TD1_OPTIONAL2_MAX_LEN>,
    pub final_check: Option<u8>,
    pub final_check_valid: Option<bool>,
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
    UnknownFormat,
    UnsupportedFormat,
    Utf8Error,
}
