//! # MRZ Parser
//! A compliance-ready library for parsing MRZs (Machine Readable Zones).

#![deny(missing_docs)]

/// Example MRZ parser function
pub fn parse_mrz(input: &str) -> Result<(), &'static str> {
    if input.is_empty() {
        Err("Input is empty")
    } else {
        Ok(())
    }
}

pub mod validation;
