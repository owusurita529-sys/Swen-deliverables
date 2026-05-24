// -------------------------------------------------------
// main.rs
// Student: [RITA OWUSU]
// Movie Festival — CSES compatible, self-contained
// -------------------------------------------------------

use std::io::{self, BufRead};

fn screen_scheduler(screenings: &[(i64, i64)]) -> usize {
    let mut programme = screenings.to_vec();
    programme.sort_by_key(|&(_, finish)| finish);

    let mut films_watched = 0;
    let mut screen_free: i64 = i64::MIN;

    for (start, finish) in programme {
        if start >= screen_free {
            films_watched += 1;
            screen_free = finish;
        }
    }

    films_watched
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // 📥 Read number of films
    let n: usize = lines
        .next().unwrap().unwrap()
        .trim()
        .parse()
        .unwrap();

    // 📥 Read start and finish for each film
    let mut screenings: Vec<(i64, i64)> = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.trim().split_whitespace();
        let start: i64 = parts.next().unwrap().parse().unwrap();
        let finish: i64 = parts.next().unwrap().parse().unwrap();
        screenings.push((start, finish));
    }

    // 🚀 Schedule maximum films!
    println!("{}", screen_scheduler(&screenings));
}