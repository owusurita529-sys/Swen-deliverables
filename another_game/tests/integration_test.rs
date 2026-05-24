// -------------------------------------------------------
// integration_test.rs
// Student: [RITA OWUSU]
// Tests for light_seeker and light_merger
// Both must agree on every single test!
// -------------------------------------------------------

use another_game::{light_seeker, light_merger};

// 🧪 Helper: runs BOTH approaches, checks they agree
fn both_agree(coin_piles: &[u64]) -> &'static str {
    let a = light_seeker(coin_piles);
    let b = light_merger(coin_piles);

    assert_eq!(
        a, b,
        "Approaches disagreed on {:?}! Seeker={} Merger={}",
        coin_piles, a, b
    );

    a
}

// ─────────────────────────────────────────────
// TEST 1: Official example — one odd pile
// Piles: 1 2 3 → first wins
// ─────────────────────────────────────────────
#[test]
fn test_official_first() {
    assert_eq!(both_agree(&[1, 2, 3]), "first");
}

// ─────────────────────────────────────────────
// TEST 2: Official example — all even
// Piles: 2 2 → second wins
// ─────────────────────────────────────────────
#[test]
fn test_official_second() {
    assert_eq!(both_agree(&[2, 2]), "second");
}

// ─────────────────────────────────────────────
// TEST 3: All odd piles → first wins
// ─────────────────────────────────────────────
#[test]
fn test_all_odd() {
    assert_eq!(both_agree(&[1, 3, 5, 7]), "first");
}

// ─────────────────────────────────────────────
// TEST 4: All even piles → second wins
// ─────────────────────────────────────────────
#[test]
fn test_all_even() {
    assert_eq!(both_agree(&[2, 4, 6, 8]), "second");
}

// ─────────────────────────────────────────────
// TEST 5: Single odd pile → first wins
// ─────────────────────────────────────────────
#[test]
fn test_single_odd_pile() {
    assert_eq!(both_agree(&[1]), "first");
}

// ─────────────────────────────────────────────
// TEST 6: Single even pile → second wins
// ─────────────────────────────────────────────
#[test]
fn test_single_even_pile() {
    assert_eq!(both_agree(&[4]), "second");
}

// ─────────────────────────────────────────────
// TEST 7: One odd pile hidden among even ones
// → first still wins!
// ─────────────────────────────────────────────
#[test]
fn test_one_odd_hidden() {
    assert_eq!(both_agree(&[2, 4, 6, 7, 8]), "first");
}

// ─────────────────────────────────────────────
// TEST 8: Large even numbers → second wins
// ─────────────────────────────────────────────
#[test]
fn test_large_even() {
    assert_eq!(both_agree(&[1000, 2000, 3000]), "second");
}

// ─────────────────────────────────────────────
// TEST 9: Large odd number → first wins
// ─────────────────────────────────────────────
#[test]
fn test_large_odd() {
    assert_eq!(both_agree(&[999, 2000, 3000]), "first");
}

// ─────────────────────────────────────────────
// TEST 10: Mix of many piles — last one is odd
// ─────────────────────────────────────────────
#[test]
fn test_last_pile_odd() {
    assert_eq!(both_agree(&[2, 4, 6, 8, 10, 3]), "first");
}