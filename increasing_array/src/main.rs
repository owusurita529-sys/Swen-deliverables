use std::io::{self, BufRead};

// -------------------------------------------------------
// Increasing Array — solved by: [RITA]
// Strategy: recursion — walk block by block, track the
// minimum height each position must reach, and add up
// all the "lifts" needed.
// -------------------------------------------------------

/// Recursively walks the staircase from left to right.
///
/// * `blocks`    – the list of numbers we're fixing
/// * `pos`       – which position we're standing on right now
/// * `floor`     – the minimum value this block must be
///                 (it must be >= the block before it)
/// * `total_ops` – running count of +1 moves used so far
fn lift_blocks(blocks: &[i64], pos: usize, floor: i64, total_ops: i64) -> i64 {
    // 🏁 We've stepped past the last block — hand back the total
    if pos == blocks.len() {
        return total_ops;
    }

    // 🧱 How tall is this block right now?
    let height = blocks[pos];

    // 🏗️ If it's shorter than the floor, we must raise it up
    let new_height = height.max(floor);

    // 📊 The cost is however much we had to raise it
    let lifts_needed = new_height - height;

    // 🔁 Move on to the next block; the new floor = new_height
    lift_blocks(blocks, pos + 1, new_height, total_ops + lifts_needed)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // 📥 First line: how many blocks are there?
    let block_count: usize = lines
        .next().unwrap().unwrap()
        .trim()
        .parse()
        .unwrap();

    // 📥 Second line: the actual block heights
    let blocks: Vec<i64> = lines
        .next().unwrap().unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .take(block_count)
        .collect();

    // 🚀 Kick off the recursion:
    //    start at position 0, floor = 0, zero moves used
    let answer = lift_blocks(&blocks, 0, 0, 0);

    println!("{}", answer);
}