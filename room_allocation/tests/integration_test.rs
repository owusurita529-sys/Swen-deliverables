// -------------------------------------------------------
// integration_test.rs
// Student: [RITA OWUSU]
// Tests for hotel_desk (BTreeMap) and
// checkout_queue (BinaryHeap) — both must agree!
// -------------------------------------------------------

use room_allocation::{hotel_desk, checkout_queue, verify_hotel};

// 🧪 Helper: runs BOTH approaches, checks they agree
fn both_agree(bookings: &[(i64, i64)]) -> usize {
    let desk = hotel_desk(bookings);
    let queue = checkout_queue(bookings);

    // Both must need same number of rooms
    assert_eq!(
        desk.rooms_needed,
        queue.rooms_needed,
        "Room count disagreed! Desk={} Queue={}",
        desk.rooms_needed, queue.rooms_needed
    );

    // Both must produce valid assignments
    assert!(
        verify_hotel(bookings, &desk),
        "hotel_desk gave invalid assignment!"
    );
    assert!(
        verify_hotel(bookings, &queue),
        "checkout_queue gave invalid assignment!"
    );

    desk.rooms_needed
}

// ─────────────────────────────────────────────
// TEST 1: Basic example — 2 rooms needed
// ─────────────────────────────────────────────
#[test]
fn test_basic_two_rooms() {
    assert_eq!(both_agree(&[(1,3),(2,5),(4,6)]), 2);
}

// ─────────────────────────────────────────────
// TEST 2: All guests overlap — need n rooms
// ─────────────────────────────────────────────
#[test]
fn test_all_overlap() {
    assert_eq!(both_agree(&[(1,10),(2,10),(3,10)]), 3);
}

// ─────────────────────────────────────────────
// TEST 3: No overlap — only 1 room needed
// ─────────────────────────────────────────────
#[test]
fn test_no_overlap() {
    assert_eq!(both_agree(&[(1,2),(3,4),(5,6)]), 1);
}

// ─────────────────────────────────────────────
// TEST 4: Single guest — 1 room
// ─────────────────────────────────────────────
#[test]
fn test_single_guest() {
    assert_eq!(both_agree(&[(1,5)]), 1);
}

// ─────────────────────────────────────────────
// TEST 5: Two guests overlap — 2 rooms
// ─────────────────────────────────────────────
#[test]
fn test_two_overlap() {
    assert_eq!(both_agree(&[(1,5),(2,6)]), 2);
}

// ─────────────────────────────────────────────
// TEST 6: Two guests no overlap — 1 room
// ─────────────────────────────────────────────
#[test]
fn test_two_no_overlap() {
    assert_eq!(both_agree(&[(1,3),(3,5)]), 1);
}

// ─────────────────────────────────────────────
// TEST 7: Many guests, many reuses
// ─────────────────────────────────────────────
#[test]
fn test_many_guests() {
    let bookings = vec![
        (1,3),(2,4),(3,5),(4,6),(5,7),
    ];
    assert_eq!(both_agree(&bookings), 2);
}

// ─────────────────────────────────────────────
// TEST 8: All same time — need n rooms
// ─────────────────────────────────────────────
#[test]
fn test_all_same_time() {
    assert_eq!(both_agree(&[(1,5),(1,5),(1,5)]), 3);
}

// ─────────────────────────────────────────────
// TEST 9: Large input — verify valid assignment
// ─────────────────────────────────────────────
#[test]
fn test_large_input() {
    let bookings: Vec<(i64, i64)> = (0..100)
        .map(|i| (i, i + 50))
        .collect();
    let result = both_agree(&bookings);
    assert!(result > 0);
}

// ─────────────────────────────────────────────
// TEST 10: Rooms reused after checkout
// 1→3, 4→6: room freed at 3, reused at 4
// ─────────────────────────────────────────────
#[test]
fn test_room_reuse() {
    assert_eq!(both_agree(&[(1,3),(4,6),(2,5)]), 2);
}