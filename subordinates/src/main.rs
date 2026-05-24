// -------------------------------------------------------
// main.rs
// Student: [RITA OWUSU]
// Subordinates — CSES compatible, fully self-contained
// -------------------------------------------------------

use std::io::{self, BufRead};
use std::collections::VecDeque;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // 📥 Read number of employees
    let n: usize = lines
        .next().unwrap().unwrap()
        .trim()
        .parse()
        .unwrap();

    // 🌳 Build the company tree
    let mut org_chart: Vec<Vec<usize>> = vec![vec![]; n + 1];

    if n > 1 {
        let boss_line = lines.next().unwrap().unwrap();
        let bosses: Vec<usize> = boss_line
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        for (i, &boss) in bosses.iter().enumerate() {
            let staff_id = i + 2;
            org_chart[boss].push(staff_id);
            org_chart[staff_id].push(boss);
        }
    }

    // 🚀 Run QueueCounter (iterative BFS)
    let tally = queue_counter(n, &org_chart);

    // 📤 Print subordinate count for each employee
    let result: Vec<String> = (1..=n)
        .map(|i| tally[i].to_string())
        .collect();
    println!("{}", result.join(" "));
}

// ── QueueCounter (BFS) ──────────────────────────────
fn queue_counter(
    headcount: usize,
    org_chart: &Vec<Vec<usize>>,
) -> Vec<usize> {
    let mut tally = vec![0usize; headcount + 1];
    let mut direct_boss = vec![0usize; headcount + 1];
    let mut reception: VecDeque<usize> = VecDeque::new();
    reception.push_back(1);
    direct_boss[1] = 0;
    let mut visit_log: Vec<usize> = Vec::new();

    while let Some(staff_id) = reception.pop_front() {
        visit_log.push(staff_id);
        for &report in &org_chart[staff_id] {
            if report != direct_boss[staff_id] {
                direct_boss[report] = staff_id;
                reception.push_back(report);
            }
        }
    }

    for &staff_id in visit_log.iter().rev() {
        let boss = direct_boss[staff_id];
        if boss != 0 {
            tally[boss] += tally[staff_id] + 1;
        }
    }
    tally
}