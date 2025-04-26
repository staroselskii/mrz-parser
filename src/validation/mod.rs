//! Validation module for MRZ fields.

//! Validation module for MRZ parsing fields and checksums.
//!
//! This module contains submodules for field validations and checksum computations.

/// Checksum-related validations and computations.
pub mod checksum;
/// Field-related validations such as alphanumeric or numeric checks.
pub mod field_validators;
