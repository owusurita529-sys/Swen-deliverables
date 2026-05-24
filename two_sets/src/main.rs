// -------------------------------------------------------
// main.rs
// Student: [RITA OWUSU]
// Two Sets — CSES compatible, fully self-contained
// -------------------------------------------------------

use std::io::{self, BufRead};

fn tray_filler(n: u64) -> Option<(Vec<u64>, Vec<u64>)> {
    let slice_total = n * (n + 1) / 2;
    if slice_total % 2 != 0 {
        return None;
    }

    let tray_goal = slice_total / 2;
    let mut tray_a: Vec<u64> = Vec::new();
    let mut tray_b: Vec<u64> = Vec::new();
    let mut still_needed = tray_goal;

    for slice in (1..=n).rev() {
        if still_needed == 0 {
            tray_b.push(slice);
        } else if slice <= still_needed {
            tray_a.push(slice);
            still_needed -= slice;
        } else {
            tray_b.push(slice);
        }
    }

    Some((tray_a, tray_b))
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: u64 = lines
        .next().unwrap().unwrap()
        .trim()
        .parse()
        .unwrap();

    match tray_filler(n) {
        None => println!("NO"),
        Some((tray_a, tray_b)) => {
            println!("YES");

            println!("{}", tray_a.len());
            let a_str: Vec<String> = tray_a
                .iter().map(|x| x.to_string()).collect();
            println!("{}", a_str.join(" "));

            println!("{}", tray_b.len());
            let b_str: Vec<String> = tray_b
                .iter().map(|x| x.to_string()).collect();
            println!("{}", b_str.join(" "));
        }
    }
}