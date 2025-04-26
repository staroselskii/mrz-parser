/// Checksum utilities for MRZ parsing.

/// Calculate the MRZ checksum for a given field.
pub fn calculate_mrz_checksum(data: &str) -> u32 {
    let weights = [7, 3, 1];
    data.chars()
        .enumerate()
        .map(|(i, c)| {
            let value = match c {
                '0'..='9' => c as u32 - '0' as u32,
                'A'..='Z' => c as u32 - 'A' as u32 + 10,
                '<' => 0,
                _ => 0, // Treat unexpected characters as 0 safely
            };
            value * weights[i % 3]
        })
        .sum::<u32>() % 10
}
