// -------------------------------------------------------
// main.rs
// Student: [RITA OWUSU]
// Another Game — CSES compatible, self-contained
// -------------------------------------------------------

use std::io::{self, BufRead};

fn light_seeker(coin_piles: &[u64]) -> &'static str {
    for &pile_size in coin_piles {
        if pile_size % 2 == 1 {
            return "first";
        }
    }
    "second"
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // 📥 Read number of test cases
    let test_cases: usize = lines
        .next().unwrap().unwrap()
        .trim()
        .parse()
        .unwrap();

    for _ in 0..test_cases {
        // 📥 Read number of piles
        let _n: usize = lines
            .next().unwrap().unwrap()
            .trim()
            .parse()
            .unwrap();

        // 📥 Read the pile sizes
        let coin_piles: Vec<u64> = lines
            .next().unwrap().unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        // 🚀 Print the winner!
        println!("{}", light_seeker(&coin_piles));
    }
}