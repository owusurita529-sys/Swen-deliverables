// -------------------------------------------------------
// main.rs
// Student: [RITA OWUSU]
// Finding Borders — CSES compatible, self-contained
// -------------------------------------------------------

use std::io::{self, BufRead};

fn ribbon_tracer(letters: &[u8]) -> Vec<usize> {
    let word_len = letters.len();
    if word_len == 0 {
        return vec![];
    }

    let mut jump = vec![0usize; word_len];
    let mut match_len = 0usize;

    for pos in 1..word_len {
        while match_len > 0 && letters[match_len] != letters[pos] {
            match_len = jump[match_len - 1];
        }
        if letters[match_len] == letters[pos] {
            match_len += 1;
        }
        jump[pos] = match_len;
    }

    let mut ribbons: Vec<usize> = Vec::new();
    let mut ribbon_len = jump[word_len - 1];

    while ribbon_len > 0 {
        ribbons.push(ribbon_len);
        ribbon_len = jump[ribbon_len - 1];
    }

    ribbons.sort();
    ribbons
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // 📥 Read the word
    let word = lines
        .next().unwrap().unwrap()
        .trim()
        .to_string();

    // 🚀 Find all ribbons using KMP
    let ribbons = ribbon_tracer(word.as_bytes());

    // 📤 Print each ribbon length
    let result: Vec<String> = ribbons
        .iter()
        .map(|x| x.to_string())
        .collect();

    println!("{}", result.join(" "));
}