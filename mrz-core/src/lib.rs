#![cfg_attr(not(feature = "std"), no_std)]

extern crate heapless;
use heapless::String;

pub const MAX_NAME_LEN: usize = 64;
pub const MAX_DOC_NUM_LEN: usize = 16;
pub const MAX_FLIGHT_NUM_LEN: usize = 8;
pub const MAX_SEAT_LEN: usize = 4;

pub mod parser;

#[derive(Debug)]
pub enum ParsedMRZ {
    ICAO(MRZICAO),
    Unknown,
}

#[derive(Debug)]
pub struct MRZICAO {
    pub document_number: String<MAX_DOC_NUM_LEN>,
    pub name: String<MAX_NAME_LEN>,
    pub birth_date: [u8; 6],
    pub expiry_date: [u8; 6],
}

#[derive(Debug, PartialEq, Eq)]
pub enum MRZFormat {
    ICAO,
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
