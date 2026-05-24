// -------------------------------------------------------
// lib.rs
// Student: [RITA OWUSU]
// Assignment: Josephus Problem I — two algorithm approaches
//
// 🎪 THE CORE IDEA:
// n children in a circle. Every 2nd child steps out.
// Track the full elimination order!
//
// APPROACH A — "RingCounter" (BTreeSet / Balanced BST):
//   Stores children in a BTreeSet (balanced BST).
//   Uses an index to track position in the circle.
//   Converts index → actual child number via BST.
//   O(n log n) — BST lookup is O(log n) per step!
//
// APPROACH B — "ChampionFinder" (Josephus formula):
//   Recursive formula: C(1)=0, C(n)=(C(n-1)+2)%n
//   Finds only the WINNER — pure maths, no simulation!
// -------------------------------------------------------

use std::collections::BTreeSet;

// ═══════════════════════════════════════════════════════
// APPROACH A — RingCounter (BTreeSet simulation)
// ═══════════════════════════════════════════════════════

/// Gets the nth element (0-indexed) from a BTreeSet
fn get_by_index(ring: &BTreeSet<u64>, idx: usize) -> u64 {
    *ring.iter().nth(idx).unwrap()
}

/// Simulates Josephus using a BTreeSet as a balanced BST.
/// Tracks position as an index into the sorted set.
/// Every 2nd person (index steps by 1) is eliminated.
///
/// * `ring_size` - number of children in the circle
pub fn ring_counter(ring_size: u64) -> Vec<u64> {
    let mut ring: BTreeSet<u64> = (1..=ring_size).collect();
    let mut stepped_out: Vec<u64> = Vec::new();

    // 🎯 Start: eliminate index 1 (second person, 0-indexed)
    let mut idx = 1usize;

    while !ring.is_empty() {
        // 🔄 Wrap index around the current ring size
        idx = idx % ring.len();

        // 🎯 Eliminate person at this index
        let eliminated = get_by_index(&ring, idx);
        stepped_out.push(eliminated);
        ring.remove(&eliminated);

        if ring.is_empty() {
            break;
        }

        // ➡️ Next index: stay at same position
        // (after removal, next person slides into this slot)
        // then skip one more (+1 for the skip)
        idx = idx % ring.len();
        idx = (idx + 1) % ring.len();
    }

    stepped_out
}

// ═══════════════════════════════════════════════════════
// APPROACH B — ChampionFinder (Josephus formula)
// ═══════════════════════════════════════════════════════

/// Finds the Josephus winner using recursive formula.
/// C(1)=0, C(n)=(C(n-1)+2)%n  (0-indexed result)
pub fn champion_finder(ring_size: u64) -> u64 {
    josephus_recur(ring_size) + 1
}

fn josephus_recur(n: u64) -> u64 {
    if n == 1 { return 0; }
    (josephus_recur(n - 1) + 2) % n
}