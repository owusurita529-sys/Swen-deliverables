// -------------------------------------------------------
// lib.rs
// Student: [RITA OWUSU]
// Assignment: Another Game — two algorithm approaches
//
// 🪙 THE CORE IDEA:
// Two players take turns removing coins from piles.
// Each turn: remove one coin from one OR two piles.
// The player who cannot move LOSES!
//
// 🏆 THE WINNING SECRET:
// If ANY pile has an odd number of coins → FIRST wins!
// If ALL piles have even coins → SECOND wins!
//
// Think of it like a light switch:
// ODD pile = light is ON 💡
// EVEN pile = light is OFF 🌑
// If ANY light is ON → first player controls the game!
//
// APPROACH A — "LightSeeker" (early exit loop):
//   Walks through piles like flicking light switches.
//   The MOMENT it finds a light ON (odd pile) → STOP!
//   First player wins! No need to check the rest.
//   Uses a simple for loop with early return.
//
// APPROACH B — "LightMerger" (bitwise fold):
//   Merges ALL piles together using bitwise OR.
//   Looks only at the last bit of each pile (odd/even).
//   Combines them all in one mathematical sweep.
//   No early exit — computationally different!
// -------------------------------------------------------

// ═══════════════════════════════════════════════════════
// APPROACH A — LightSeeker (early exit loop)
// ═══════════════════════════════════════════════════════

/// Walks through piles looking for any odd one.
/// Stops immediately when it finds an odd pile —
/// like finding a light switch that's ON!
///
/// * `coin_piles` - the number of coins in each pile
///
/// Returns "first" or "second"
pub fn light_seeker(coin_piles: &[u64]) -> &'static str {
    for &pile_size in coin_piles {
        // 💡 Is this pile's light ON? (odd = remainder 1)
        if pile_size % 2 == 1 {
            // 🏆 Found it! First player wins!
            return "first";
        }
    }
    // 🌑 All lights off — second player wins!
    "second"
}

// ═══════════════════════════════════════════════════════
// APPROACH B — LightMerger (bitwise fold)
// ═══════════════════════════════════════════════════════

/// Merges all pile sizes using bitwise OR on last bit.
/// The last bit of a number = 1 if odd, 0 if even.
/// OR-ing them all together: if ANY is 1, result is 1!
/// Processes every pile — no early exit at all!
///
/// * `coin_piles` - the number of coins in each pile
///
/// Returns "first" or "second"
pub fn light_merger(coin_piles: &[u64]) -> &'static str {
    // 🔦 Collect all the "last bits" into one value
    // pile & 1 = last bit (1 if odd, 0 if even)
    // fold with OR: any 1 anywhere = result is 1
    let any_light_on = coin_piles
        .iter()
        .fold(0u64, |merged, &pile_size| {
            merged | (pile_size & 1)
        });

    // 💡 Was any light on?
    if any_light_on == 1 {
        "first"
    } else {
        "second"
    }
}