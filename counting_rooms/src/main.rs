// -------------------------------------------------------
// main.rs
// Student: [RITA OWUSU]
// Counting Rooms — CSES compatible, self-contained
// -------------------------------------------------------

use std::io::{self, BufRead};
use std::collections::VecDeque;

const STEPS: [(i32, i32); 4] = [
    (-1, 0), (1, 0), (0, -1), (0, 1),
];

fn count_with_sweeper(blueprint: &Vec<Vec<char>>) -> usize {
    let rows = blueprint.len();
    let cols = blueprint[0].len();
    let mut footprint = vec![vec![false; cols]; rows];
    let mut room_tally = 0;

    for r in 0..rows {
        for c in 0..cols {
            if blueprint[r][c] == '.' && !footprint[r][c] {
                room_tally += 1;
                footprint[r][c] = true;

                let mut wave: VecDeque<(usize, usize)> =
                    VecDeque::new();
                wave.push_back((r, c));

                while let Some((row, col)) = wave.pop_front() {
                    for (step_r, step_c) in STEPS {
                        let peek_row = row as i32 + step_r;
                        let peek_col = col as i32 + step_c;

                        if peek_row < 0
                            || peek_col < 0
                            || peek_row >= rows as i32
                            || peek_col >= cols as i32
                        {
                            continue;
                        }

                        let nr = peek_row as usize;
                        let nc = peek_col as usize;

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

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // 📥 Read grid dimensions
    let first_line = lines.next().unwrap().unwrap();
    let mut dims = first_line.trim().split_whitespace();
    let rows: usize = dims.next().unwrap().parse().unwrap();
    let _cols: usize = dims.next().unwrap().parse().unwrap();

    // 📥 Read the blueprint
    let mut blueprint: Vec<Vec<char>> = Vec::new();
    for _ in 0..rows {
        let line = lines.next().unwrap().unwrap();
        blueprint.push(line.trim().chars().collect());
    }

    // 🚀 Count the rooms!
    println!("{}", count_with_sweeper(&blueprint));
}