// -------------------------------------------------------
// main.rs
// Student: [YOUR NAME]
// Josephus Problem I — CSES compatible
// Uses Fenwick Tree for O(log n) kth element lookup!
// -------------------------------------------------------

use std::io::{self, BufRead, Write, BufWriter};

// 🌳 Fenwick Tree (Binary Indexed Tree)
// Lets us find the kth active child in O(log n)!
struct CircleTree {
    data: Vec<usize>,
    size: usize,
}

impl CircleTree {
    fn new(n: usize) -> Self {
        let mut tree = CircleTree {
            data: vec![0; n + 1],
            size: n,
        };
        for i in 1..=n {
            tree.update(i, 1);
        }
        tree
    }

    fn update(&mut self, mut i: usize, delta: i64) {
        while i <= self.size {
            self.data[i] = (self.data[i] as i64 + delta) as usize;
            i += i & i.wrapping_neg();
        }
    }

    fn find_kth(&self, mut k: usize) -> usize {
        let mut pos = 0;
        let mut log = 1;
        while log <= self.size {
            log <<= 1;
        }
        log >>= 1;
        while log > 0 {
            if pos + log <= self.size
                && self.data[pos + log] < k
            {
                k -= self.data[pos + log];
                pos += log;
            }
            log >>= 1;
        }
        pos + 1
    }
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    let mut lines = stdin.lock().lines();

    let circle_size: usize = lines
        .next().unwrap().unwrap()
        .trim()
        .parse()
        .unwrap();

    let mut tree = CircleTree::new(circle_size);
    let mut stepped_out: Vec<usize> = Vec::new();

    let mut current_idx = 2usize;
    let mut remaining = circle_size;

    while remaining > 0 {
        if current_idx > remaining {
            current_idx = ((current_idx - 1) % remaining) + 1;
        }

        let child = tree.find_kth(current_idx);
        stepped_out.push(child);
        tree.update(child, -1);
        remaining -= 1;

        if remaining == 0 { break; }

        if current_idx > remaining {
            current_idx = 1;
        }
        current_idx = (current_idx % remaining) + 1;
    }

    let output: Vec<String> = stepped_out
        .iter()
        .map(|x| x.to_string())
        .collect();
    writeln!(out, "{}", output.join(" ")).unwrap();
}
