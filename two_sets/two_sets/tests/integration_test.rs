// -------------------------------------------------------
// integration_test.rs
// Student: [RITA OWUSU]
// Tests for tray_filler (greedy) and tray_builder (mathematical)
// Both must agree on every single test!
// -------------------------------------------------------

use two_sets::{tray_filler, tray_builder, verify_trays};

// 🧪 Helper: runs BOTH approaches, checks they both
// agree on whether a split is possible, and if so,
// that both produce valid equal-sum splits
fn both_agree(n: u64) -> bool {
    let result_a = tray_filler(n);
    let result_b = tray_builder(n);

    // Both must agree on possibility
    assert_eq!(
        result_a.is_some(),
        result_b.is_some(),
        "Approaches disagreed on possibility for n={}", n
    );

    // If possible, both must produce valid splits
    if let Some(ref r) = result_a {
        assert!(verify_trays(r),
            "tray_filler gave invalid split for n={}", n);
    }
    if let Some(ref r) = result_b {
        assert!(verify_trays(r),
            "tray_builder gave invalid split for n={}", n);
    }

    result_a.is_some()
}

// ─────────────────────────────────────────────
// TEST 1: n=1 → total=1 (odd) → NO
// ─────────────────────────────────────────────
#[test]
fn test_n1_impossible() {
    assert!(!both_agree(1));
}

// ─────────────────────────────────────────────
// TEST 2: n=2 → total=3 (odd) → NO
// ─────────────────────────────────────────────
#[test]
fn test_n2_impossible() {
    assert!(!both_agree(2));
}

// ─────────────────────────────────────────────
// TEST 3: n=3 → total=6 (even) → YES
// Group A: 3+2+1=6, each tray=3
// ─────────────────────────────────────────────
#[test]
fn test_n3_possible() {
    assert!(both_agree(3));
}

// ─────────────────────────────────────────────
// TEST 4: n=4 → total=10 (even) → YES
// Group A: 4+1=5, Group B: 3+2=5
// ─────────────────────────────────────────────
#[test]
fn test_n4_possible() {
    assert!(both_agree(4));

    let result = tray_filler(4).unwrap();
    let sum_a: u64 = result.tray_a.iter().sum();
    let sum_b: u64 = result.tray_b.iter().sum();
    assert_eq!(sum_a, 5);
    assert_eq!(sum_b, 5);
}

// ─────────────────────────────────────────────
// TEST 5: n=5 → total=15 (odd) → NO
// ─────────────────────────────────────────────
#[test]
fn test_n5_impossible() {
    assert!(!both_agree(5));
}

// ─────────────────────────────────────────────
// TEST 6: n=6 → total=21 (odd) → NO
// ─────────────────────────────────────────────
#[test]
fn test_n6_impossible() {
    assert!(!both_agree(6));
}

// ─────────────────────────────────────────────
// TEST 7: n=7 → total=28 (even) → YES
// The official example from CSES!
// ─────────────────────────────────────────────
#[test]
fn test_n7_official_example() {
    assert!(both_agree(7));

    let result = tray_filler(7).unwrap();
    let sum_a: u64 = result.tray_a.iter().sum();
    let sum_b: u64 = result.tray_b.iter().sum();
    assert_eq!(sum_a, 14);
    assert_eq!(sum_b, 14);
}

// ─────────────────────────────────────────────
// TEST 8: n=8 → total=36 (even) → YES
// You worked this out yourself earlier!
// ─────────────────────────────────────────────
#[test]
fn test_n8_possible() {
    assert!(both_agree(8));

    let result = tray_filler(8).unwrap();
    let sum_a: u64 = result.tray_a.iter().sum();
    let sum_b: u64 = result.tray_b.iter().sum();
    assert_eq!(sum_a, 18);
    assert_eq!(sum_b, 18);
}

// ─────────────────────────────────────────────
// TEST 9: Pattern check — n%4==0 or n%4==3 → YES
// All others → NO
// ─────────────────────────────────────────────
#[test]
fn test_pattern_holds() {
    for n in 1..=20u64 {
        let possible = both_agree(n);
        let total = n * (n + 1) / 2;
        let expected = total % 2 == 0;
        assert_eq!(
            possible, expected,
            "Pattern failed for n={}", n
        );
    }
}

// ─────────────────────────────────────────────
// TEST 10: Large n=100 → should work fast!
// ─────────────────────────────────────────────
#[test]
fn test_large_n() {
    assert!(both_agree(100));

    let result = tray_filler(100).unwrap();
    let sum_a: u64 = result.tray_a.iter().sum();
    let sum_b: u64 = result.tray_b.iter().sum();
    assert_eq!(sum_a, sum_b);
}