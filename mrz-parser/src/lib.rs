#![cfg_attr(not(feature = "std"), no_std)]
pub use mrz_core::{MRZFormat, MRZParseError, ParsedMRZ};

#[cfg(feature = "std")]
pub use mrz_host::{
    parse_lines,
    parse_mrz_date_with_reference,
    MRZ, // Rich enum
};

#[cfg(feature = "uniffi")]
pub mod uniffi_api;
#[cfg(feature = "uniffi")]
uniffi_macros::setup_scaffolding!();
