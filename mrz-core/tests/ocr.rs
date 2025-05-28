use heapless::String;
use mrz_core::ocr::{ocr_permutations, ocr_substitutions};

#[test]
fn test_ocr_substitutions() {
    assert_eq!(ocr_substitutions('O'), ['0']);
    assert_eq!(ocr_substitutions('0'), ['O']);
    assert_eq!(ocr_substitutions('S'), ['5']);
    assert_eq!(ocr_substitutions('5'), ['S']);
    assert_eq!(ocr_substitutions('I'), ['1']);
    assert_eq!(ocr_substitutions('1'), ['I']);
    assert_eq!(ocr_substitutions('A'), []);
}

#[test]
fn test_ocr_permutations_single_error() {
    let input: String<9> = String::from("X5Z98765O");
    let expected = "X5Z987650";
    let corrected = ocr_permutations::<9, 16>(&input, 2);
    assert!(corrected.iter().any(|s| s == expected));
}

#[test]
fn test_ocr_permutations_multiple_candidates() {
    let input: String<4> = String::from("S5O1");
    let corrected = ocr_permutations::<4, 16>(&input, 2);
    assert!(corrected.iter().any(|s| s == "5501"));
    assert!(corrected.iter().any(|s| s == "S501"));
    assert!(corrected.iter().any(|s| s == "S5O1"));
}

#[test]
fn test_ocr_permutations_no_substitution() {
    let input: String<8> = String::from("12345678");
    let corrected = ocr_permutations::<8, 16>(&input, 2);
    assert!(corrected.iter().any(|s| s == "12345678"));
}

#[test]
fn test_ocr_permutations_with_deep_substitution() {
    let input: String<5> = String::from("S5O1I");
    let corrected = ocr_permutations::<5, 64>(&input, 3);
    assert!(corrected.iter().any(|s| s == "55011"));
    assert!(corrected.iter().any(|s| s == "S5011"));
    assert!(corrected.iter().any(|s| s == "S5O1I"));
}

#[test]
fn test_ocr_permutations_exhaustive_limit() {
    let input: String<4> = String::from("S5O1");
    let corrected = ocr_permutations::<4, 2>(&input, 1);
    assert!(
        corrected.len() <= 2,
        "Should not exceed the M=2 permutation limit"
    );
}

#[test]
#[should_panic(expected = "N must be greater than 0")]
fn test_ocr_permutations_empty_input() {
    let input: String<0> = String::from("");
    let _ = ocr_permutations::<0, 2>(&input, 1);
}

#[test]
fn test_ocr_permutations_all_substitutable() {
    let input: String<6> = String::from("O0S5I1");
    let corrected = ocr_permutations::<6, 64>(&input, 3);
    assert!(corrected.iter().any(|s| s == "005511"));
    assert!(corrected.iter().any(|s| s == "O0S5I1"));
}

#[test]
fn test_ocr_permutations_repeated_substitutable() {
    let input: String<6> = String::from("OOSSOO");
    let corrected = ocr_permutations::<6, 1024>(&input, 6);
    assert!(
        corrected.iter().any(|s| s == "00SS00"),
        "Expected '00SS00' to be in permutations, got: {:?}",
        corrected
    );
}

#[test]
fn test_ocr_permutations_edges() {
    let input: String<5> = String::from("O123I");
    let corrected = ocr_permutations::<5, 16>(&input, 2);
    assert!(corrected.iter().any(|s| s == "01231"));
}

#[test]
fn test_ocr_permutations_max_depth_limit() {
    let input: String<4> = String::from("O5I1");
    let corrected = ocr_permutations::<4, 64>(&input, 1);
    assert!(!corrected.iter().any(|s| s == "0511"));
}

#[test]
fn test_ocr_permutations_no_candidates() {
    let input: String<4> = String::from("EFGH");
    let corrected = ocr_permutations::<4, 16>(&input, 2);
    assert!(
        corrected.iter().all(|s| s == "EFGH"),
        "Expected only original string 'EFGH', got: {:?}",
        corrected
    );
}
#[test]
fn test_ocr_substitutions_debug_abcd() {
    println!("Substitutions for A: {:?}", ocr_substitutions('A'));
    println!("Substitutions for B: {:?}", ocr_substitutions('B'));
    println!("Substitutions for C: {:?}", ocr_substitutions('C'));
    println!("Substitutions for D: {:?}", ocr_substitutions('D'));
    // Ensure they are truly empty if no substitutions are expected
    assert!(ocr_substitutions('A').is_empty());
    assert_eq!(ocr_substitutions('B'), ['8']);
    assert!(ocr_substitutions('C').is_empty());
    assert!(ocr_substitutions('D').is_empty());
}

#[test]
fn test_ocr_permutations_debug_00ss00() {
    let input: String<6> = String::from("OOSSOO");
    let corrected = ocr_permutations::<6, 1024>(&input, 6);
    println!(
        "Generated permutations for OOSSOO (len={}): {:?}",
        corrected.len(),
        corrected
    );
    assert!(corrected.iter().any(|s| s == "00SS00"));
}

#[test]
fn test_ocr_permutations_nested_errors() {
    let input: String<4> = String::from("O5I1");
    let corrected = ocr_permutations::<4, 64>(&input, 3);
    assert!(corrected.iter().any(|s| s == "0511"));
    assert!(corrected.iter().any(|s| s == "O511"));
    assert!(corrected.iter().any(|s| s == "O5I1"));
}

#[test]
fn test_ocr_permutations_exceeding_max_depth() {
    let input: String<4> = String::from("O5I1");
    let corrected = ocr_permutations::<4, 64>(&input, 0);
    assert_eq!(corrected.len(), 1);
    assert_eq!(corrected[0], "O5I1");
}

#[test]
fn test_ocr_permutations_minimal_input() {
    let input: String<1> = String::from("O");
    let corrected = ocr_permutations::<1, 4>(&input, 1);
    assert!(corrected.iter().any(|s| s == "0"));
    assert!(corrected.iter().any(|s| s == "O"));
}

#[test]
fn test_ocr_permutations_large_input_with_few_substitutions() {
    let input: String<16> = String::from("ABCD1234EFGH5678");
    let corrected = ocr_permutations::<16, 128>(&input, 2);
    assert!(corrected.iter().any(|s| s == "ABCD1234EFGH5678"));
}

#[test]
fn test_ocr_permutations_all_identical_substitutable() {
    let input: String<4> = String::from("OOOO");
    let corrected = ocr_permutations::<4, 128>(&input, 4);
    println!(
        "Generated permutations for 'OOOO' (count: {}): {:?}",
        corrected.len(),
        corrected
    );
    assert!(
        corrected.iter().any(|s| s == "0000"),
        "Expected '0000' in permutations, got: {:?}",
        corrected
    );
    assert!(
        corrected.iter().any(|s| s == "OOOO"),
        "Expected original 'OOOO' in permutations"
    );
}
