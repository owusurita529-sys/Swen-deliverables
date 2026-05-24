// -------------------------------------------------------
// lib.rs — shared logic, exposed for testing
// -------------------------------------------------------

/// Recursively walks the staircase from left to right.
///
/// * `blocks`    – the list of numbers we're fixing
/// * `pos`       – which position we're standing on right now  
/// * `floor`     – the minimum value this block must be
/// * `total_ops` – running count of +1 moves used so far
pub fn lift_blocks(blocks: &[i64], pos: usize, floor: i64, total_ops: i64) -> i64 {
    if pos == blocks.len() {
        return total_ops;
    }

    let height = blocks[pos];
    let new_height = height.max(floor);
    let lifts_needed = new_height - height;

    lift_blocks(blocks, pos + 1, new_height, total_ops + lifts_needed)
}