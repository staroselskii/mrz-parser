#![cfg_attr(not(feature = "std"), no_std)]
pub use mrz_core::{MRZFormat, MRZParseError, ParsedMRZ};

#[cfg(feature = "std")]
pub use mrz_host::{
    parse_lines,
    parse_mrz_date,
    MRZ,     // Rich enum
    MrzIcaoTd3, // Rich types
};
