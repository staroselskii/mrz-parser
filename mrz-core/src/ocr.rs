//! OCR Substitution Utilities
//!
//! This module provides helper functions and logic to support Optical Character Recognition (OCR)
//! post-processing, including common character substitution mappings (e.g., 'O' <-> '0', 'S' <-> '5')
//! and permutation generation of possible corrected strings.
//!
//! The primary use case is recovering plausible original strings from OCR output by exploring
//! a defined set of character-level substitutions.

use heapless::Deque;
use heapless::FnvIndexSet;
use heapless::String;
use heapless::Vec;

/// Returns a list of plausible OCR substitution characters for a given character.
///
/// This function provides a symmetric mapping, e.g. both 'O' -> ['0'] and '0' -> ['O'].
///
/// # Examples
///
/// ```
/// use mrz_core::ocr::ocr_substitutions;
/// assert_eq!(ocr_substitutions('O'), heapless::Vec::<char, 4>::from_slice(&['0']).unwrap());
/// assert_eq!(ocr_substitutions('X'), heapless::Vec::<char, 4>::from_slice(&[]).unwrap());
/// ```
pub fn ocr_substitutions(c: char) -> Vec<char, 4> {
    let mut substitutes = Vec::new();
    match c {
        'O' => {
            substitutes.push('0').ok();
        }
        '0' => {
            substitutes.push('O').ok();
        }
        'I' => {
            substitutes.push('1').ok();
        }
        '1' => {
            substitutes.push('I').ok();
        }
        'S' => {
            substitutes.push('5').ok();
        }
        '5' => {
            substitutes.push('S').ok();
        }
        'B' => {
            substitutes.push('8').ok();
        }
        '8' => {
            substitutes.push('B').ok();
        }
        _ => {}
    }
    substitutes
}

/// Generates permutations of a string by applying common OCR substitutions.
///
/// This function uses a breadth-first search approach to generate all strings up to a certain
/// substitution depth. Each substitution replaces a single character with one of its plausible
/// OCR alternatives.
///
/// # Parameters
///
/// - `input`: The input string to permute.
/// - `max_depth`: The maximum number of sequential substitutions applied to generate permutations.
///
/// # Type Parameters
///
/// - `N`: The maximum length of the input string.
/// - `M`: The maximum number of permutations generated.
///
/// # Returns
///
/// A vector of string permutations including the original string.
///
/// # Panics
///
/// This function may panic if `N == 0` or `M == 0`.
///
/// # Examples
///
/// ```
/// use mrz_core::ocr::ocr_permutations;
/// let perms = ocr_permutations::<6, 64>("OSSO", 2);
/// assert!(perms.iter().any(|s| s.contains("0SS")));
/// ```
///
/// Note: To guarantee inclusion of deeply substituted variants (e.g., "0000" from "OOOO"),
/// ensure that `M` is large enough to contain all permutations up to `max_depth` substitutions.
/// The number of generated permutations can grow exponentially with both `max_depth` and the number of
/// substitutable characters in the input.
pub fn ocr_permutations<const N: usize, const M: usize>(
    input: &str,
    max_depth: usize,
) -> Vec<String<N>, M> {
    assert!(N > 0, "N must be greater than 0");
    assert!(M > 0, "M must be greater than 0");
    assert!(input.len() <= N, "Input length exceeds buffer limit");
    let mut results = Vec::<String<N>, M>::new();
    let mut queue = Deque::<(String<N>, usize), M>::new();
    let mut seen = FnvIndexSet::<String<N>, M>::new();

    // Initialize BFS queue with the original string and depth 0.
    // Use a set to track seen permutations and avoid duplicates.
    let input_string: String<N> = String::from(input);
    queue.push_back((input_string.clone(), 0)).ok();
    seen.insert(input_string.clone()).ok();

    // BFS ensures minimal substitutions are considered first.
    // Note: High `max_depth` values may exponentially increase permutation count.
    // Recommend limiting `max_depth` to 4–6 for 6–9 character inputs.
    while let Some((current, depth)) = queue.pop_front() {
        results.push(current.clone()).ok();

        if depth >= max_depth {
            continue;
        }

        // Attempt substitutions at each character position.
        for i in 0..current.len() {
            match current.chars().nth(i) {
                Some(ch) => {
                    let subs = ocr_substitutions(ch);
                    if subs.is_empty() {
                        continue;
                    }
                    for subst in subs.iter().copied() {
                        let mut chars: Vec<char, N> = current.chars().collect();
                        if let Some(c) = chars.get_mut(i) {
                            *c = subst;
                        }
                        let new_str: String<N> = chars.iter().collect::<String<N>>();

                        // Only proceed if this permutation hasn't been seen.
                        if seen.insert(new_str.clone()).is_ok() {
                            queue.push_back((new_str, depth + 1)).ok();
                        }
                    }
                }
                None => continue,
            }
        }
    }

    results
}
