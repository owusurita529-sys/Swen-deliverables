// -------------------------------------------------------
// integration_test.rs
// Student: [RITA OWUSU]
// Tests for ring_counter (BTreeSet) and
// champion_finder (formula) — must agree on winner!
// -------------------------------------------------------

use josephus::{ring_counter, champion_finder};

// 🧪 Helper: runs ring_counter and checks the
// last element (winner) matches champion_finder
fn check_winner(ring_size: u64) -> u64 {
    let order = ring_counter(ring_size);

    // Last person standing = winner
    let winner = *order.last().unwrap();

    // Champion finder must agree on the winner!
    let formula_winner = champion_finder(ring_size);
    assert_eq!(
        winner, formula_winner,
        "Winner disagreed for n={}! \
         Simulation={} Formula={}",
        ring_size, winner, formula_winner
    );

    winner
}

// ─────────────────────────────────────────────
// TEST 1: Official CSES example n=7
// Expected order: 2 4 6 1 5 3 7
// ─────────────────────────────────────────────
#[test]
fn test_official_example() {
    let order = ring_counter(7);
    assert_eq!(order, vec![2, 4, 6, 1, 5, 3, 7]);
}

// ─────────────────────────────────────────────
// TEST 2: n=1 → only one child, they win!
// ─────────────────────────────────────────────
#[test]
fn test_single_child() {
    let order = ring_counter(1);
    assert_eq!(order, vec![1]);
    assert_eq!(champion_finder(1), 1);
}

// ─────────────────────────────────────────────
// TEST 3: n=2 → child 2 eliminated first, child 1 wins
// ─────────────────────────────────────────────
#[test]
fn test_two_children() {
    let order = ring_counter(2);
    assert_eq!(order, vec![2, 1]);
    assert_eq!(champion_finder(2), 1);
}

// ─────────────────────────────────────────────
// TEST 4: n=5 → check full order
// Expected: 2 4 1 5 3
// ─────────────────────────────────────────────
#[test]
fn test_five_children() {
    let order = ring_counter(5);
    assert_eq!(order, vec![2, 4, 1, 5, 3]);
}

// ─────────────────────────────────────────────
// TEST 5: Winner check for n=3
// ─────────────────────────────────────────────
#[test]
fn test_winner_n3() {
    assert_eq!(check_winner(3), 3);
}

// ─────────────────────────────────────────────
// TEST 6: Winner check for n=6
// ─────────────────────────────────────────────
#[test]
fn test_winner_n6() {
    assert_eq!(check_winner(6), 5);
}

// ─────────────────────────────────────────────
// TEST 7: Order length equals ring size
// Every child steps out exactly once!
// ─────────────────────────────────────────────
#[test]
fn test_order_length() {
    for n in [1, 5, 10, 20] {
        let order = ring_counter(n);
        assert_eq!(
            order.len(), n as usize,
            "Wrong length for n={}", n
        );
    }
}

// ─────────────────────────────────────────────
// TEST 8: All children appear exactly once
// No duplicates, no missing children!
// ─────────────────────────────────────────────
#[test]
fn test_all_children_appear() {
    let order = ring_counter(10);
    let mut sorted = order.clone();
    sorted.sort();
    assert_eq!(sorted, vec![1,2,3,4,5,6,7,8,9,10]);
}

// ─────────────────────────────────────────────
// TEST 9: Formula and simulation agree for n=1..20
// ─────────────────────────────────────────────
#[test]
fn test_formula_agrees_with_simulation() {
    for n in 1..=20 {
        check_winner(n);
    }
}

// ─────────────────────────────────────────────
// TEST 10: Large n=100 — winner check
// ─────────────────────────────────────────────
#[test]
fn test_large_n_winner() {
    check_winner(100);
}