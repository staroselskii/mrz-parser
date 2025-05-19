#![cfg_attr(not(feature = "std"), no_std)]

extern crate heapless;
use heapless::String;

pub mod parser;

#[derive(Debug)]
pub enum ParsedMRZ {
    MrzIcaoTd3(MrzIcaoTd3),
    Unknown,
}

pub const ICAO_TD3_DOC_NUM_MAX_LEN: usize = 9;
pub const ICAO_TD3_NAME_MAX_LEN: usize = 39;
pub const ICAO_COMMON_DATE_LEN: usize = 6;

#[derive(Debug)]
pub struct MrzIcaoTd3 {
    pub document_number: String<ICAO_TD3_DOC_NUM_MAX_LEN>,
    pub name: String<ICAO_TD3_NAME_MAX_LEN>,
    pub birth_date: [u8; ICAO_COMMON_DATE_LEN],
    pub birth_date_check: u8,
    pub expiry_date: [u8; ICAO_COMMON_DATE_LEN],
    pub expiry_date_check: u8,
}

pub const ICAO_TD1_DOC_NUM_MAX_LEN: usize = 9;
pub const ICAO_TD1_NAME_MAX_LEN: usize = 30;
pub const ICAO_TD1_OPTIONAL1_MAX_LEN: usize = 2;
pub const ICAO_TD1_OPTIONAL2_MAX_LEN: usize = 30;
pub const ICAO_COMMON_DOC_CODE_LEN: usize = 2;
pub const ICAO_COMMON_COUNTRY_CODE_LEN: usize = 3;

#[derive(Debug)]
pub struct MrzIcaoTd1 {
    pub document_code: [u8; ICAO_COMMON_DOC_CODE_LEN],
    pub issuing_state: [u8; ICAO_COMMON_COUNTRY_CODE_LEN],
    pub name: String<ICAO_TD1_NAME_MAX_LEN>,
    pub document_number: String<ICAO_TD1_DOC_NUM_MAX_LEN>,
    pub document_number_check: u8,
    pub nationality: [u8; ICAO_COMMON_COUNTRY_CODE_LEN],
    pub birth_date: [u8; ICAO_COMMON_DATE_LEN],
    pub birth_date_check: u8,
    pub sex: u8,
    pub expiry_date: [u8; ICAO_COMMON_DATE_LEN],
    pub expiry_date_check: u8,
    pub optional_data1: String<ICAO_TD1_OPTIONAL1_MAX_LEN>,
    pub optional_data2: String<ICAO_TD1_OPTIONAL2_MAX_LEN>,
    pub final_check: u8,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MRZFormat {
    MrzIcaoTd3,
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
