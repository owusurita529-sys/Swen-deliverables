// -------------------------------------------------------
// lib.rs
// Student: [RITA OWUSU]
// Assignment: Movie Festival — two algorithm approaches
//
// 🎬 THE CORE IDEA:
// You're at a film festival with overlapping movies.
// Watch as many as possible — one at a time!
//
// 🧠 THE HEURISTIC (Greedy Rule):
// Always pick the movie that ends SOONEST!
// Finishing early = more time for future movies!
//
// APPROACH A — "ScreenScheduler" (Sort by end + Greedy):
//   Sorts all films by finish time — earliest first.
//   Walks the sorted list picking non-clashing films.
//   Like a critic with a sorted programme! 📋
//   O(n log n) sort + O(n) scan!
//
// APPROACH B — "FilmSweeper" (Sort by end + index scan):
//   Also sorts by end time but uses an explicit index
//   pointer instead of a for loop iterator.
//   Tracks position manually — computationally different!
//   Same heuristic, different mechanical approach!
// -------------------------------------------------------

// ═══════════════════════════════════════════════════════
// APPROACH A — ScreenScheduler (Sort + for loop)
// ═══════════════════════════════════════════════════════

/// Schedules maximum films using greedy sort approach.
/// Sorts by finish time, uses for loop to pick films.
///
/// * `screenings` - list of (start, finish) time pairs
pub fn screen_scheduler(screenings: &[(i64, i64)]) -> usize {
    // 🎬 Sort films by finish time — earliest finisher first!
    let mut programme = screenings.to_vec();
    programme.sort_by_key(|&(_, finish)| finish);

    let mut films_watched = 0;
    let mut screen_free: i64 = i64::MIN;

    // 🔁 For loop — iterator based scan
    for (start, finish) in programme {
        if start >= screen_free {
            films_watched += 1;
            screen_free = finish;
        }
    }

    films_watched
}

// ═══════════════════════════════════════════════════════
// APPROACH B — FilmSweeper (Sort + index pointer scan)
// ═══════════════════════════════════════════════════════

/// Schedules maximum films using index pointer scan.
/// Sorts by finish time, uses while loop with explicit
/// index pointer — mechanically different from for loop!
///
/// * `screenings` - list of (start, finish) time pairs
pub fn film_sweeper(screenings: &[(i64, i64)]) -> usize {
    // 🎬 Sort films by finish time — same heuristic!
    let mut programme = screenings.to_vec();
    programme.sort_by_key(|&(_, finish)| finish);

    let mut films_watched = 0;
    let mut screen_free: i64 = i64::MIN;

    // 🔁 While loop with explicit index — different mechanic!
    let mut idx = 0;
    while idx < programme.len() {
        let (start, finish) = programme[idx];
        if start >= screen_free {
            films_watched += 1;
            screen_free = finish;
        }
        idx += 1;
    }

    films_watched
}