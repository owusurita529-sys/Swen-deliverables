// -------------------------------------------------------
// lib.rs
// Student: [RITA OWUSU]
// Assignment: Counting Rooms — two algorithm approaches
//
// 🏠 THE CORE IDEA:
// A building map is a grid of floor (.) and wall (#).
// A room = a group of connected floor squares.
// You can only walk up, down, left, right — not diagonal!
//
// APPROACH A — "FloodDiver" (Recursive DFS):
//   Finds an unvisited floor tile and dives deep,
//   marking every connected tile as it goes.
//   Like a mouse exploring a maze — one path at a time!
//
// APPROACH B — "FloodSweeper" (Iterative BFS):
//   Finds an unvisited floor tile and sweeps outward
//   in all directions using a queue — like a wave!
//   No recursion — purely a spreading wave effect!
// -------------------------------------------------------

use std::collections::VecDeque;

// 🧭 The 4 directions we can step: up, down, left, right
const STEPS: [(i32, i32); 4] = [
    (-1,  0), // 👆 up
    ( 1,  0), // 👇 down
    ( 0, -1), // 👈 left
    ( 0,  1), // 👉 right
];

// ═══════════════════════════════════════════════════════
// APPROACH A — FloodDiver (Recursive DFS)
// ═══════════════════════════════════════════════════════

/// Recursively marks all connected floor tiles as visited.
/// Dives deep in one direction before trying others.
/// Like a mouse that always goes as far as it can first!
///
/// * `blueprint` - the building map grid
/// * `footprint` - tracks which tiles we've walked on
/// * `row`       - current tile row
/// * `col`       - current tile column
pub fn flood_diver(
    blueprint: &Vec<Vec<char>>,
    footprint: &mut Vec<Vec<bool>>,
    row: usize,
    col: usize,
) {
    // 👣 Leave a footprint on this tile
    footprint[row][col] = true;

    // 🔍 Check all 4 neighbouring tiles
    for (step_r, step_c) in STEPS {
        let peek_row = row as i32 + step_r;
        let peek_col = col as i32 + step_c;

        // 🚧 Don't walk off the edge of the map!
        if peek_row < 0
            || peek_col < 0
            || peek_row >= blueprint.len() as i32
            || peek_col >= blueprint[0].len() as i32
        {
            continue;
        }

        let nr = peek_row as usize;
        let nc = peek_col as usize;

        // 🐭 Dive in if it's floor and we haven't been there
        if blueprint[nr][nc] == '.' && !footprint[nr][nc] {
            flood_diver(blueprint, footprint, nr, nc);
        }
    }
}

/// Counts rooms using recursive DFS.
/// Every unvisited floor tile found = one new room!
///
/// * `blueprint` - the building map grid
pub fn count_with_diver(blueprint: &Vec<Vec<char>>) -> usize {
    let rows = blueprint.len();
    let cols = blueprint[0].len();
    let mut footprint = vec![vec![false; cols]; rows];
    let mut room_tally = 0;

    for r in 0..rows {
        for c in 0..cols {
            // 🏠 New unvisited floor tile = new room!
            if blueprint[r][c] == '.' && !footprint[r][c] {
                room_tally += 1;
                flood_diver(blueprint, &mut footprint, r, c);
            }
        }
    }

    room_tally
}

// ═══════════════════════════════════════════════════════
// APPROACH B — FloodSweeper (Iterative BFS)
// ═══════════════════════════════════════════════════════

/// Counts rooms using iterative BFS.
/// Spreads outward from each new room like a wave.
/// Uses a queue — no recursion needed at all!
///
/// * `blueprint` - the building map grid
pub fn count_with_sweeper(blueprint: &Vec<Vec<char>>) -> usize {
    let rows = blueprint.len();
    let cols = blueprint[0].len();
    let mut footprint = vec![vec![false; cols]; rows];
    let mut room_tally = 0;

    for r in 0..rows {
        for c in 0..cols {
            // 🏠 New unvisited floor tile = new room!
            if blueprint[r][c] == '.' && !footprint[r][c] {
                room_tally += 1;
                footprint[r][c] = true;

                // 🌊 Start the wave from this tile
                let mut wave: VecDeque<(usize, usize)> =
                    VecDeque::new();
                wave.push_back((r, c));

                // 🌊 Keep spreading until wave dies out
                while let Some((row, col)) = wave.pop_front() {
                    for (step_r, step_c) in STEPS {
                        let peek_row = row as i32 + step_r;
                        let peek_col = col as i32 + step_c;

                        // 🚧 Don't walk off the map!
                        if peek_row < 0
                            || peek_col < 0
                            || peek_row >= rows as i32
                            || peek_col >= cols as i32
                        {
                            continue;
                        }

                        let nr = peek_row as usize;
                        let nc = peek_col as usize;

                        // 🌊 Add to wave if floor and unvisited
                        if blueprint[nr][nc] == '.'
                            && !footprint[nr][nc]
                        {
                            footprint[nr][nc] = true;
                            wave.push_back((nr, nc));
                        }
                    }
                }
            }
        }
    }

    room_tally
}