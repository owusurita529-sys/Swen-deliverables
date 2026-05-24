// -------------------------------------------------------
// lib.rs
// Student: [RITA OWUSU]
// Assignment: Subordinates — two algorithm approaches
//
// 🏢 THE CORE IDEA:
// A company is shaped like a family tree.
// Employee 1 is the big boss at the top.
// Count how many people are BELOW each employee.
//
// APPROACH A — "BranchCounter" (Recursive DFS):
//   Starts at the top and dives into every branch.
//   Counts workers on the way BACK UP to the top.
//   Like a manager asking each team lead:
//   "How many people are under you?"
//   Then adding them all up!
//
// APPROACH B — "QueueCounter" (Iterative BFS):
//   Uses a waiting queue — like a reception desk.
//   Processes employees level by level.
//   Then tallies counts from bottom to top.
//   Purely iterative — no self-calling at all!
// -------------------------------------------------------

use std::collections::VecDeque;

// ═══════════════════════════════════════════════════════
// APPROACH A — BranchCounter (Recursive DFS)
// ═══════════════════════════════════════════════════════

/// Recursively counts subordinates for each employee.
/// Dives deep into every branch, then counts up
/// on the way back to the root.
///
/// * `staff_id`  - current employee being visited
/// * `boss_id`   - who we came from (don't go back!)
/// * `org_chart` - company tree as adjacency list
/// * `tally`     - stores each employee's subordinate count
pub fn branch_counter(
    staff_id: usize,
    boss_id: usize,
    org_chart: &Vec<Vec<usize>>,
    tally: &mut Vec<usize>,
) {
    // 👥 Visit every direct report under this employee
    for &report in &org_chart[staff_id] {
        // 🚫 Skip — don't walk back to our own boss!
        if report == boss_id {
            continue;
        }

        // 🔽 Dive into this report's branch first
        branch_counter(report, staff_id, org_chart, tally);

        // ⬆️ Coming back up — add their team size + 1
        // (+1 counts the report themselves!)
        tally[staff_id] += tally[report] + 1;
    }
}

// ═══════════════════════════════════════════════════════
// APPROACH B — QueueCounter (Iterative BFS)
// ═══════════════════════════════════════════════════════

/// Iteratively counts subordinates using a queue.
/// Visits employees level by level like a reception desk —
/// first come, first served. Then tallies from leaves up.
///
/// * `headcount` - total number of employees
/// * `org_chart` - company tree as adjacency list
///
/// Returns a Vec of subordinate counts per employee
pub fn queue_counter(
    headcount: usize,
    org_chart: &Vec<Vec<usize>>,
) -> Vec<usize> {
    let mut tally = vec![0usize; headcount + 1];
    let mut direct_boss = vec![0usize; headcount + 1];

    // 🎫 Reception desk queue — director goes first!
    let mut reception: VecDeque<usize> = VecDeque::new();
    reception.push_back(1);
    direct_boss[1] = 0;

    // 📋 Log of who we visited and in what order
    let mut visit_log: Vec<usize> = Vec::new();

    // 🚶 Process employees level by level
    while let Some(staff_id) = reception.pop_front() {
        visit_log.push(staff_id);

        for &report in &org_chart[staff_id] {
            if report != direct_boss[staff_id] {
                direct_boss[report] = staff_id;
                reception.push_back(report);
            }
        }
    }

    // ⬆️ Tally from bottom to top
    // Reverse log = leaves first, director last
    for &staff_id in visit_log.iter().rev() {
        let boss = direct_boss[staff_id];
        if boss != 0 {
            tally[boss] += tally[staff_id] + 1;
        }
    }

    tally
}