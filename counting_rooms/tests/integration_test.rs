// -------------------------------------------------------
// integration_test.rs
// Student: [RITA OWUSU]
// Tests for count_with_diver (DFS) and
// count_with_sweeper (BFS) — both must always agree!
// -------------------------------------------------------

use counting_rooms::{count_with_diver, count_with_sweeper};

// 🧪 Helper: parses a map string, runs BOTH approaches,
// checks they agree, returns the room count
fn both_agree(map: &[&str]) -> usize {
    let blueprint: Vec<Vec<char>> = map
        .iter()
        .map(|row| row.chars().collect())
        .collect();

    let dfs_count = count_with_diver(&blueprint);
    let bfs_count = count_with_sweeper(&blueprint);

    assert_eq!(
        dfs_count, bfs_count,
        "Approaches disagreed! DFS={} BFS={}",
        dfs_count, bfs_count
    );

    dfs_count
}

// ─────────────────────────────────────────────
// TEST 1: Official CSES example → 3 rooms
// ─────────────────────────────────────────────
#[test]
fn test_official_example() {
    let map = &[
        "########",
        "#..#...#",
        "####.###",
        "#..#...#",
        "########",
    ];
    assert_eq!(both_agree(map), 3);
}

// ─────────────────────────────────────────────
// TEST 2: All walls — zero rooms
// ─────────────────────────────────────────────
#[test]
fn test_all_walls() {
    let map = &[
        "###",
        "###",
        "###",
    ];
    assert_eq!(both_agree(map), 0);
}

// ─────────────────────────────────────────────
// TEST 3: All floor — one big room
// ─────────────────────────────────────────────
#[test]
fn test_all_floor() {
    let map = &[
        "...",
        "...",
        "...",
    ];
    assert_eq!(both_agree(map), 1);
}

// ─────────────────────────────────────────────
// TEST 4: Single floor tile — one room
// ─────────────────────────────────────────────
#[test]
fn test_single_floor_tile() {
    let map = &[
        "###",
        "#.#",
        "###",
    ];
    assert_eq!(both_agree(map), 1);
}

// ─────────────────────────────────────────────
// TEST 5: Two isolated rooms
// ─────────────────────────────────────────────
#[test]
fn test_two_isolated_rooms() {
    let map = &[
        "#####",
        "#.#.#",
        "#####",
    ];
    assert_eq!(both_agree(map), 2);
}

// ─────────────────────────────────────────────
// TEST 6: Long corridor — still one room
// ─────────────────────────────────────────────
#[test]
fn test_long_corridor() {
    let map = &[
        "#######",
        "#.....#",
        "#######",
    ];
    assert_eq!(both_agree(map), 1);
}

// ─────────────────────────────────────────────
// TEST 7: Four corner rooms
// ─────────────────────────────────────────────
#[test]
fn test_four_corners() {
    let map = &[
        ".#.",
        "###",
        ".#.",
    ];
    assert_eq!(both_agree(map), 4);
}

// ─────────────────────────────────────────────
// TEST 8: Zigzag corridor — still one room
// ─────────────────────────────────────────────
#[test]
fn test_zigzag_corridor() {
    let map = &[
        "#####",
        "#...#",
        "###.#",
        "#...#",
        "#####",
    ];
    assert_eq!(both_agree(map), 1);
}

// ─────────────────────────────────────────────
// TEST 9: Single row of alternating floor/wall
// ─────────────────────────────────────────────
#[test]
fn test_single_row_alternating() {
    let map = &[
        ".#.#.#.",
    ];
    assert_eq!(both_agree(map), 4);
}

// ─────────────────────────────────────────────
// TEST 10: Single column of alternating floor/wall
// ─────────────────────────────────────────────
#[test]
fn test_single_column_alternating() {
    let map = &[
        ".",
        "#",
        ".",
        "#",
        ".",
    ];
    assert_eq!(both_agree(map), 3);
}