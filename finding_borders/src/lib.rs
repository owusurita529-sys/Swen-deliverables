// -------------------------------------------------------
// lib.rs
// Student: [RITA OWUSU]
// Assignment: Finding Borders — two algorithm approaches
//
// 🔤 THE CORE IDEA:
// A border = a piece that appears at BOTH the start
// and the end of a word (but isn't the whole word!)
//
// Think of it like a ribbon on a gift box —
// the same pattern wraps around both ends!
//
// APPROACH A — "RibbonTracer" (KMP failure function):
//   Builds a smart jump table in one single pass.
//   Each slot says: "longest matching ribbon here"
//   Then follows the chain to collect all ribbons.
//   Super fast: O(n) — never looks at a letter twice!
//
// APPROACH B — "RibbonChecker" (Brute force):
//   Tries every possible ribbon length one by one.
//   Peeks at the front and back — do they match?
//   Simple and honest: O(n²) — checks everything!
//   Totally different computation from RibbonTracer!
// -------------------------------------------------------

// ═══════════════════════════════════════════════════════
// APPROACH A — RibbonTracer (KMP failure function)
// ═══════════════════════════════════════════════════════

/// Finds all border lengths using the KMP failure table.
/// Builds a jump table then follows the chain of borders
/// from the end of the string back to the beginning.
///
/// * `letters` - the input word as bytes
///
/// Returns sorted list of border lengths
pub fn ribbon_tracer(letters: &[u8]) -> Vec<usize> {
    let word_len = letters.len();
    if word_len == 0 {
        return vec![];
    }

    // 🗂️ Build the jump table
    // jump[i] = length of longest ribbon ending at pos i
    let mut jump = vec![0usize; word_len];
    let mut match_len = 0usize;

    for pos in 1..word_len {
        // 🔄 Shrink match until letters align or we hit zero
        while match_len > 0 && letters[match_len] != letters[pos] {
            match_len = jump[match_len - 1];
        }
        // ✅ Found a matching letter — grow the match!
        if letters[match_len] == letters[pos] {
            match_len += 1;
        }
        jump[pos] = match_len;
    }

    // 🎀 Follow the ribbon chain from the end
    let mut ribbons: Vec<usize> = Vec::new();
    let mut ribbon_len = jump[word_len - 1];

    while ribbon_len > 0 {
        ribbons.push(ribbon_len);
        ribbon_len = jump[ribbon_len - 1];
    }

    // 📤 Sort ascending so shortest ribbon comes first
    ribbons.sort();
    ribbons
}

// ═══════════════════════════════════════════════════════
// APPROACH B — RibbonChecker (Brute force)
// ═══════════════════════════════════════════════════════

/// Finds all border lengths by checking every possibility.
/// For each length from 1 to n-1, peeks at the front
/// and back of the word — if they match, it's a ribbon!
///
/// * `letters` - the input word as bytes
///
/// Returns sorted list of border lengths
pub fn ribbon_checker(letters: &[u8]) -> Vec<usize> {
    let word_len = letters.len();
    if word_len == 0 {
        return vec![];
    }

    let mut ribbons: Vec<usize> = Vec::new();

    // 🔍 Try every possible ribbon length 1..n-1
    for ribbon_len in 1..word_len {
        // 👀 Peek at front and back slices
        let front = &letters[..ribbon_len];
        let back  = &letters[word_len - ribbon_len..];

        // 🎀 Front matches back = valid ribbon!
        if front == back {
            ribbons.push(ribbon_len);
        }
    }

    // Already in ascending order — no need to sort!
    ribbons
}