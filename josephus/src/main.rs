use std::collections::BTreeSet;
use std::io::{self, BufRead};

fn get_by_index(ring: &BTreeSet<u64>, idx: usize) -> u64 {
    *ring.iter().nth(idx).unwrap()
}

fn ring_counter(ring_size: u64) -> Vec<u64> {
    let mut ring: BTreeSet<u64> = (1..=ring_size).collect();
    let mut stepped_out: Vec<u64> = Vec::new();
    let mut idx = 1usize;

    while !ring.is_empty() {
        idx = idx % ring.len();
        let eliminated = get_by_index(&ring, idx);
        stepped_out.push(eliminated);
        ring.remove(&eliminated);

        if ring.is_empty() { break; }

        idx = idx % ring.len();
        idx = (idx + 1) % ring.len();
    }

    stepped_out
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: u64 = lines
        .next().unwrap().unwrap()
        .trim()
        .parse()
        .unwrap();

    let order = ring_counter(n);
    let result: Vec<String> = order
        .iter()
        .map(|x| x.to_string())
        .collect();
    println!("{}", result.join(" "));
}