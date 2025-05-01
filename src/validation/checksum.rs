//! MRZ checksum calculations.

/// Calculates MRZ checksum using weighting factors 7, 3, and 1.
pub fn calculate_mrz_checksum(data: &str) -> u32 {
    let weights = [7, 3, 1];
    data.chars()
        .zip(weights.iter().cycle())
        .map(|(c, w)| char_value(c) * w)
        .sum::<u32>() % 10
}

/// Helper function to convert MRZ character to numeric value.
fn char_value(c: char) -> u32 {
    match c {
        '0'..='9' => c as u32 - '0' as u32,
        'A'..='Z' => c as u32 - 'A' as u32 + 10,
        '<' => 0,
        _ => 0, // invalid characters treated as 0
    }
}
