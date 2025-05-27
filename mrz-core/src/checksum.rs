/// Computes the ICAO checksum for a given MRZ field.
///
/// This function uses the standard weighting scheme defined in ICAO Doc 9303
/// (weights of 7, 3, and 1 repeated cyclically) and supports digits (`0`–`9`),
/// uppercase letters (`A`–`Z`), and the filler character `<`.
///
/// Returns `Some(checksum_digit)` if all characters are valid, otherwise `None`.
///
/// # Examples
///
/// ```
/// use mrz_core::checksum::compute_checksum;
///
/// let data = b"L898902C3"; // Example document number
/// let checksum = compute_checksum(data);
/// assert_eq!(checksum, Some(6));
/// ```
pub fn compute_checksum(data: &[u8]) -> Option<u8> {
    fn char_value(c: u8) -> Option<u8> {
        match c {
            b'0'..=b'9' => Some(c - b'0'),
            b'A'..=b'Z' => Some(c - b'A' + 10),
            b'<' => Some(0),
            _ => None,
        }
    }

    let weights = [7, 3, 1];
    let mut sum: u32 = 0;

    for (i, &b) in data.iter().enumerate() {
        let val = char_value(b)?;
        let weight = weights[i % 3];
        sum += val as u32 * weight as u32;
    }

    Some((sum % 10) as u8)
}
