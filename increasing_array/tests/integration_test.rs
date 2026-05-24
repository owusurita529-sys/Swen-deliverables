// -------------------------------------------------------
// integration_test.rs
// Student: [RITA OWUSU]
// Tests for lift_blocks (recursive increasing array)
// -------------------------------------------------------

use array_lifter::lift_blocks;

// 🧪 Helper: runs lift_blocks cleanly
fn check(blocks: &[i64]) -> i64 {
    lift_blocks(blocks, 0, 0, 0)
}

// ─────────────────────────────────────────────
// TEST 1: Official CSES example
// Array: 3 2 5 1 7 → expected: 5
// ─────────────────────────────────────────────
#[test]
fn test_official_example() {
    assert_eq!(check(&[3, 2, 5, 1, 7]), 5);
}

// ─────────────────────────────────────────────
// TEST 2: Already increasing — no moves needed
// ─────────────────────────────────────────────
#[test]
fn test_already_increasing() {
    assert_eq!(check(&[1, 2, 3, 4, 5]), 0);
}

// ─────────────────────────────────────────────
// TEST 3: All equal — no moves needed
// ─────────────────────────────────────────────
#[test]
fn test_all_equal() {
    assert_eq!(check(&[5, 5, 5, 5]), 0);
}

// ─────────────────────────────────────────────
// TEST 4: Fully decreasing — hardest case
// 5 4 3 2 1 → costs 1+2+3+4 = 10
// ─────────────────────────────────────────────
#[test]
fn test_fully_decreasing() {
    assert_eq!(check(&[5, 4, 3, 2, 1]), 10);
}

// ─────────────────────────────────────────────
// TEST 5: Single element — zero moves
// ─────────────────────────────────────────────
#[test]
fn test_single_element() {
    assert_eq!(check(&[42]), 0);
}

// ─────────────────────────────────────────────
// TEST 6: Two elements, second smaller
// 10 3 → need to add 7
// ─────────────────────────────────────────────
#[test]
fn test_two_elements_drop() {
    assert_eq!(check(&[10, 3]), 7);
}

// ─────────────────────────────────────────────
// TEST 7: Two elements already correct
// 3 9 → no moves needed
// ─────────────────────────────────────────────
#[test]
fn test_two_elements_correct() {
    assert_eq!(check(&[3, 9]), 0);
}

// ─────────────────────────────────────────────
// TEST 8: Big drop in the middle
// 1 1 100 2 2 → 2 needs +98, 2 needs +98 = 196
// ─────────────────────────────────────────────
#[test]
fn test_big_drop_in_middle() {
    assert_eq!(check(&[1, 1, 100, 2, 2]), 196);
}

// ─────────────────────────────────────────────
// TEST 9: All zeros — no moves needed
// ─────────────────────────────────────────────
#[test]
fn test_all_zeros() {
    assert_eq!(check(&[0, 0, 0, 0]), 0);
}

// ─────────────────────────────────────────────
// TEST 10: Large drop at end
// 1 2 3 4 1 → need to add 3 to last
// ─────────────────────────────────────────────
#[test]
fn test_drop_at_end() {
    assert_eq!(check(&[1, 2, 3, 4, 1]), 3);
}