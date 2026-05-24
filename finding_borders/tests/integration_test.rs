// -------------------------------------------------------
// integration_test.rs
// Student: [RITA OWUSU]
// Tests for ribbon_tracer (KMP) and ribbon_checker (Naive)
// Both must agree on every single test!
// -------------------------------------------------------

use finding_borders::{ribbon_tracer, ribbon_checker};

// 🧪 Helper: runs BOTH approaches, checks they agree
fn both_agree(word: &str) -> Vec<usize> {
    let letters = word.as_bytes();
    let kmp    = ribbon_tracer(letters);
    let naive  = ribbon_checker(letters);

    assert_eq!(
        kmp, naive,
        "Approaches disagreed on '{}'! KMP={:?} Naive={:?}",
        word, kmp, naive
    );

    kmp
}

// ─────────────────────────────────────────────
// TEST 1: Official CSES example
// "abcababcab" → borders at length 2 and 5
// ─────────────────────────────────────────────
#[test]
fn test_official_example() {
    assert_eq!(both_agree("abcababcab"), vec![2, 5]);
}

// ─────────────────────────────────────────────
// TEST 2: Simple two-letter border
// "aabaa" → borders at length 1 and 2
// ─────────────────────────────────────────────
#[test]
fn test_aabaa() {
    assert_eq!(both_agree("aabaa"), vec![1, 2]);
}

// ─────────────────────────────────────────────
// TEST 3: No borders at all
// "abc" → no prefix matches suffix
// ─────────────────────────────────────────────
#[test]
fn test_no_borders() {
    assert_eq!(both_agree("abc"), vec![]);
}

// ─────────────────────────────────────────────
// TEST 4: All same letters
// "aaaa" → borders at 1, 2, 3
// ─────────────────────────────────────────────
#[test]
fn test_all_same_letters() {
    assert_eq!(both_agree("aaaa"), vec![1, 2, 3]);
}

// ─────────────────────────────────────────────
// TEST 5: Single letter — no borders possible
// ─────────────────────────────────────────────
#[test]
fn test_single_letter() {
    assert_eq!(both_agree("a"), vec![]);
}

// ─────────────────────────────────────────────
// TEST 6: Two same letters
// "aa" → border at length 1
// ─────────────────────────────────────────────
#[test]
fn test_two_same_letters() {
    assert_eq!(both_agree("aa"), vec![1]);
}

// ─────────────────────────────────────────────
// TEST 7: Two different letters — no border
// "ab" → no border
// ─────────────────────────────────────────────
#[test]
fn test_two_different_letters() {
    assert_eq!(both_agree("ab"), vec![]);
}

// ─────────────────────────────────────────────
// TEST 8: Repeating pattern
// "abababab" → borders at 2, 4, 6
// ─────────────────────────────────────────────
#[test]
fn test_repeating_pattern() {
    assert_eq!(both_agree("abababab"), vec![2, 4, 6]);
}

// ─────────────────────────────────────────────
// TEST 9: Only first and last match
// "abcda" → border at length 1 only
// ─────────────────────────────────────────────
#[test]
fn test_only_first_last_match() {
    assert_eq!(both_agree("abcda"), vec![1]);
}

// ─────────────────────────────────────────────
// TEST 10: Longer repeating word
// "aaaaaa" → borders at 1,2,3,4,5
// ─────────────────────────────────────────────
#[test]
fn test_long_repeating() {
    assert_eq!(both_agree("aaaaaa"), vec![1, 2, 3, 4, 5]);
}