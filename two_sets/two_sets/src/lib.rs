// -------------------------------------------------------
// lib.rs
// Student: [RITA OWUSU]
// Two Sets — two computationally different approaches
//
// 🍕 THE CORE IDEA:
// Split numbers 1..n into two groups of equal sum.
// Total sum = n*(n+1)/2 — must be even to split equally.
//
// APPROACH A — "TrayFiller" (Greedy top-down):
//   Counts DOWN from n, grabs numbers into Tray A
//   until the tray is exactly full. Greedy and simple!
//   Makes decisions one number at a time.
//
// APPROACH B — "TrayBuilder" (Mathematical pairs):
//   Works from BOTH ends simultaneously.
//   Two pointers walking inward — purely mathematical!
//   No greedy decisions needed!
// -------------------------------------------------------

/// The two groups after a successful split
#[derive(Debug, Clone, PartialEq)]
pub struct TrayResult {
    pub tray_a: Vec<u64>,
    pub tray_b: Vec<u64>,
}

// ═══════════════════════════════════════════════════════
// APPROACH A — TrayFiller (Greedy top-down)
// ═══════════════════════════════════════════════════════

/// Greedily fills Tray A by counting down from n.
/// Picks each number if it fits in the remaining space.
/// Everything that doesn't fit goes to Tray B.
///
/// Returns None if an equal split is impossible.
pub fn tray_filler(n: u64) -> Option<TrayResult> {
    let slice_total = n * (n + 1) / 2;
    if slice_total % 2 != 0 {
        return None;
    }

    let tray_goal = slice_total / 2;
    let mut tray_a: Vec<u64> = Vec::new();
    let mut tray_b: Vec<u64> = Vec::new();
    let mut still_needed = tray_goal;

    for slice in (1..=n).rev() {
        if still_needed == 0 {
            tray_b.push(slice);
        } else if slice <= still_needed {
            tray_a.push(slice);
            still_needed -= slice;
        } else {
            tray_b.push(slice);
        }
    }

    Some(TrayResult { tray_a, tray_b })
}

// ═══════════════════════════════════════════════════════
// APPROACH B — TrayBuilder (Mathematical pairs)
// ═══════════════════════════════════════════════════════

/// Builds the two trays by walking from both ends inward.
/// At each step, decides which tray gets the current
/// number based purely on the running sum — no guessing!
///
/// Returns None if an equal split is impossible.
pub fn tray_builder(n: u64) -> Option<TrayResult> {
    let slice_total = n * (n + 1) / 2;
    if slice_total % 2 != 0 {
        return None;
    }

    let tray_goal = slice_total / 2;
    let mut tray_a: Vec<u64> = Vec::new();
    let mut tray_b: Vec<u64> = Vec::new();

    let mut left_ptr = 1u64;
    let mut right_ptr = n;
    let mut running_sum: u64 = 0;

    while left_ptr <= right_ptr {
        if left_ptr == right_ptr {
            if running_sum + left_ptr <= tray_goal {
                tray_a.push(left_ptr);
            } else {
                tray_b.push(left_ptr);
            }
            break;
        }

        if running_sum + right_ptr <= tray_goal {
            tray_a.push(right_ptr);
            running_sum += right_ptr;
            if running_sum + left_ptr <= tray_goal {
                tray_a.push(left_ptr);
                running_sum += left_ptr;
            } else {
                tray_b.push(left_ptr);
            }
        } else {
            tray_b.push(right_ptr);
            tray_b.push(left_ptr);
        }

        left_ptr += 1;
        right_ptr -= 1;
    }

    Some(TrayResult { tray_a, tray_b })
}

// ═══════════════════════════════════════════════════════
// SHARED HELPER — double-check a split is correct
// ═══════════════════════════════════════════════════════

/// Verifies both trays have equal sums — used in tests!
pub fn verify_trays(result: &TrayResult) -> bool {
    let sum_a: u64 = result.tray_a.iter().sum();
    let sum_b: u64 = result.tray_b.iter().sum();
    sum_a == sum_b
}