// -------------------------------------------------------
// integration_test.rs
// Student: [RITA OWUSU]
// Tests for screen_scheduler and film_sweeper
// Both must agree on every single test!
// -------------------------------------------------------

use movie_festival::{screen_scheduler, film_sweeper};

// 🧪 Helper: runs BOTH approaches, checks they agree
fn both_agree(screenings: &[(i64, i64)]) -> usize {
    let a = screen_scheduler(screenings);
    let b = film_sweeper(screenings);

    assert_eq!(
        a, b,
        "Approaches disagreed! Scheduler={} Sweeper={}",
        a, b
    );

    a
}

// ─────────────────────────────────────────────
// TEST 1: Basic example — 2 films
// ─────────────────────────────────────────────
#[test]
fn test_basic_example() {
    assert_eq!(both_agree(&[(3,5),(1,4),(4,7),(6,9)]), 2);
}

// ─────────────────────────────────────────────
// TEST 2: No overlap — watch all films
// ─────────────────────────────────────────────
#[test]
fn test_no_overlap() {
    assert_eq!(both_agree(&[(1,2),(3,4),(5,6),(7,8)]), 4);
}

// ─────────────────────────────────────────────
// TEST 3: All overlap — watch only 1
// ─────────────────────────────────────────────
#[test]
fn test_all_overlap() {
    assert_eq!(both_agree(&[(1,10),(2,10),(3,10)]), 1);
}

// ─────────────────────────────────────────────
// TEST 4: Single film — watch it!
// ─────────────────────────────────────────────
#[test]
fn test_single_film() {
    assert_eq!(both_agree(&[(1,5)]), 1);
}

// ─────────────────────────────────────────────
// TEST 5: Two films no overlap — watch both
// ─────────────────────────────────────────────
#[test]
fn test_two_no_overlap() {
    assert_eq!(both_agree(&[(1,3),(3,5)]), 2);
}

// ─────────────────────────────────────────────
// TEST 6: Two films overlap — watch only 1
// ─────────────────────────────────────────────
#[test]
fn test_two_overlap() {
    assert_eq!(both_agree(&[(1,5),(2,6)]), 1);
}

// ─────────────────────────────────────────────
// TEST 7: Chain of films — watch them all!
// ─────────────────────────────────────────────
#[test]
fn test_chain_of_films() {
    assert_eq!(both_agree(&[(1,2),(2,3),(3,4),(4,5)]), 4);
}

// ─────────────────────────────────────────────
// TEST 8: Greedy picks short film over long
// Short: 1→3, Long: 1→10, After: 3→5
// Greedy picks short (ends 3), then after (ends 5) = 2
// If we picked long we'd only get 1
// ─────────────────────────────────────────────
#[test]
fn test_greedy_wins() {
    assert_eq!(both_agree(&[(1,3),(1,10),(3,5)]), 2);
}

// ─────────────────────────────────────────────
// TEST 9: Large input — all non-overlapping
// ─────────────────────────────────────────────
#[test]
fn test_large_no_overlap() {
    let screenings: Vec<(i64, i64)> = (0..100)
        .map(|i| (i * 2, i * 2 + 1))
        .collect();
    assert_eq!(both_agree(&screenings), 100);
}

// ─────────────────────────────────────────────
// TEST 10: Large input — all overlap
// ─────────────────────────────────────────────
#[test]
fn test_large_all_overlap() {
    let screenings: Vec<(i64, i64)> = (0..100)
        .map(|i| (i, 200))
        .collect();
    assert_eq!(both_agree(&screenings), 1);
}