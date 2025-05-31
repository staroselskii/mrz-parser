use crate::checked_field::CheckedField;
use crate::checksum::compute_checksum;
use crate::ocr::ocr_permutations;
use crate::{MRZChecksumError, MRZParseError};

use core::str::FromStr;

/// Tries to correct a field with OCR permutations until the checksum is valid.
///
/// # Parameters
/// - `raw`: The original, potentially corrupted field string.
/// - `expected_checksum`: The expected checksum character.
/// - `max_depth`: The maximum depth of OCR permutation corrections.
/// - `field_kind`: The specific `MRZChecksumError` to return on failure.
///
/// # Returns
/// `CheckedField<T>` if a corrected permutation passes the checksum and parses successfully,
/// or an appropriate `MRZParseError` if none match.
pub fn correct_checked_field<const N: usize, const M: usize, T>(
    raw: &str,
    expected_checksum: char,
    max_depth: usize,
    field_kind: MRZChecksumError,
) -> Result<CheckedField<T>, MRZParseError>
where
    T: FromStr,
    <T as FromStr>::Err: core::fmt::Debug,
{
    let cleaned = raw.trim_end_matches('<');

    let permutations = ocr_permutations::<N, M>(cleaned, max_depth);
    #[cfg(test)]
    dbg!(&permutations);

    let expected = expected_checksum
        .to_digit(10)
        .ok_or(MRZParseError::from_checksum(field_kind.clone()))? as u8;

    for p in permutations {
        if compute_checksum(p.as_bytes()) == Some(expected) {
            if let Ok(value) = T::from_str(&p) {
                return Ok(CheckedField::new(value, None, b'0' + expected));
            }
        }
    }

    Err(MRZParseError::from_checksum(field_kind))
}
