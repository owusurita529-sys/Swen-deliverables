// -------------------------------------------------------
// lib.rs
// Student: [RITA OWUSU]
// Assignment: Room Allocation — two algorithm approaches
//
// 🏨 THE CORE IDEA:
// Guests check in and out of a hotel.
// Assign rooms using as few as possible!
// A room is free when checkout time <= new arrival time.
//
// APPROACH A — "HotelDesk" (BTreeMap / B-Tree):
//   Keeps a sorted B-Tree of checkout times.
//   Scans for earliest free room using range query!
//   B-Tree range query = O(log n) per guest!
//
// APPROACH B — "CheckoutQueue" (BinaryHeap):
//   Keeps a min-heap of (checkout_time, room_number).
//   Peeks at soonest checkout — free if <= arrival!
//   Heap peek = O(1), pop/push = O(log n)!
// -------------------------------------------------------

use std::collections::{BTreeMap, BinaryHeap};
use std::cmp::Reverse;

/// Result of a hotel room assignment
#[derive(Debug, Clone, PartialEq)]
pub struct HotelResult {
    pub rooms_needed: usize,
    pub room_for_guest: Vec<usize>,
}

// ═══════════════════════════════════════════════════════
// APPROACH A — HotelDesk (BTreeMap / B-Tree)
// ═══════════════════════════════════════════════════════

pub fn hotel_desk(bookings: &[(i64, i64)]) -> HotelResult {
    let mut guest_list: Vec<(i64, i64, usize)> = bookings
        .iter()
        .enumerate()
        .map(|(i, &(a, d))| (a, d, i))
        .collect();
    guest_list.sort();

    // 🌳 B-Tree board: checkout_time → room_number
    let mut checkout_board: BTreeMap<i64, usize> = BTreeMap::new();
    let mut room_log = vec![0usize; bookings.len()];
    let mut room_tally = 1usize;

    for (checkin, checkout, guest_id) in guest_list {
        // 🔍 Find room freed at or before checkin
        let vacancy = checkout_board
            .range(..=checkin)
            .next()
            .map(|(&time, &room)| (time, room));

        let assigned = if let Some((time, room)) = vacancy {
            // ✅ Found a free room — reuse it!
            checkout_board.remove(&time);
            room
        } else {
            // 🆕 No vacancy — open a new room!
            let r = room_tally;
            room_tally += 1;
            r
        };

        room_log[guest_id] = assigned;
        checkout_board.insert(checkout, assigned);
    }

    HotelResult {
        rooms_needed: room_tally - 1,
        room_for_guest: room_log,
    }
}

// ═══════════════════════════════════════════════════════
// APPROACH B — CheckoutQueue (BinaryHeap)
// ═══════════════════════════════════════════════════════

pub fn checkout_queue(bookings: &[(i64, i64)]) -> HotelResult {
    let mut guest_list: Vec<(i64, i64, usize)> = bookings
        .iter()
        .enumerate()
        .map(|(i, &(a, d))| (a, d, i))
        .collect();
    guest_list.sort();

    // 🏔️ Min-heap: (checkout_time, room_number)
    let mut checkout_heap: BinaryHeap<Reverse<(i64, usize)>> =
        BinaryHeap::new();
    let mut room_log = vec![0usize; bookings.len()];
    let mut room_tally = 1usize;

    for (checkin, checkout, guest_id) in guest_list {
        let assigned = if let Some(&Reverse((earliest, room))) =
            checkout_heap.peek()
        {
            if earliest <= checkin {
                // ✅ Room freed at or before checkin!
                checkout_heap.pop();
                room
            } else {
                // 🆕 All rooms busy — open a new one!
                let r = room_tally;
                room_tally += 1;
                r
            }
        } else {
            // 🆕 Hotel empty — open first room!
            let r = room_tally;
            room_tally += 1;
            r
        };

        room_log[guest_id] = assigned;
        checkout_heap.push(Reverse((checkout, assigned)));
    }

    HotelResult {
        rooms_needed: room_tally - 1,
        room_for_guest: room_log,
    }
}

// ═══════════════════════════════════════════════════════
// SHARED HELPER — verify no two guests share a room
// ═══════════════════════════════════════════════════════

pub fn verify_hotel(
    bookings: &[(i64, i64)],
    result: &HotelResult,
) -> bool {
    let n = bookings.len();
    for i in 0..n {
        for j in (i + 1)..n {
            if result.room_for_guest[i] == result.room_for_guest[j] {
                let (a1, d1) = bookings[i];
                let (a2, d2) = bookings[j];
                if a1 < d2 && a2 < d1 {
                    return false;
                }
            }
        }
    }
    true
}