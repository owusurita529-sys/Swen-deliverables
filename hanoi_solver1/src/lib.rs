// -------------------------------------------------------
// lib.rs
// Student: [RITA OWUSU]
// Assignment: Tower of Hanoi — two algorithm approaches
//
// 🗼 THE CORE IDEA:
// Imagine moving a stack of rings from one pillar to another.
// You can only move ONE ring at a time, and you can NEVER
// place a bigger ring on top of a smaller one.
//
// APPROACH A — "RingShifter" (Recursive):
//   Solves the puzzle by trusting a simple rule:
//   To move N rings, first move N-1 out of the way,
//   move the biggest ring, then stack N-1 back on top.
//   Each call handles one layer of the problem.
//
// APPROACH B — "RingMachine" (Iterative with task pile):
//   Never calls itself. Instead keeps a pile of jobs.
//   Pulls one job at a time, breaks it into smaller jobs,
//   pushes those back onto the pile. Like a robot with
//   a to-do list — purely mechanical, no self-calling!
// -------------------------------------------------------

/// One ring movement: lifted from one pillar to another
#[derive(Debug, Clone, PartialEq)]
pub struct RingMove {
    pub from: &'static str,
    pub to:   &'static str,
}

// ═══════════════════════════════════════════════════════
// APPROACH A — RingShifter (Recursive)
// ═══════════════════════════════════════════════════════

/// Solves Tower of Hanoi recursively — splits into 3 steps:
/// 1. Shift n-1 rings to the spare pillar
/// 2. Move the biggest ring to the destination
/// 3. Shift n-1 rings from spare to destination
///
/// * `n`       - number of rings to move
/// * `source`  - pillar we're moving rings FROM
/// * `dest`    - pillar we're moving rings TO
/// * `spare`   - the extra pillar we can borrow
/// * `journal` - running log of every ring movement
pub fn ring_shifter(
    n: u64,
    source: &'static str,
    dest:   &'static str,
    spare:  &'static str,
    journal: &mut Vec<RingMove>,
) {
    // 🏁 Only one ring left — move it straight across!
    if n == 1 {
        journal.push(RingMove { from: source, to: dest });
        return;
    }

    // 🔄 Phase 1: clear the top n-1 rings onto spare
    ring_shifter(n - 1, source, spare, dest, journal);

    // 🏋️ Phase 2: slide the giant bottom ring to dest
    journal.push(RingMove { from: source, to: dest });

    // 🔄 Phase 3: stack the n-1 rings from spare to dest
    ring_shifter(n - 1, spare, dest, source, journal);
}


// ═══════════════════════════════════════════════════════
// APPROACH B — RingMachine (Iterative with task pile)
// ═══════════════════════════════════════════════════════

/// A job card for our RingMachine robot.
/// Either: figure out how to move n rings (needs planning)
///     or: just move one ring right now (action!)
#[derive(Debug)]
enum JobCard {
    /// Still needs planning — break it down further
    PlanMove {
        n:      u64,
        source: &'static str,
        dest:   &'static str,
        spare:  &'static str,
    },
    /// Ready to execute — move one ring directly
    ExecuteMove {
        from: &'static str,
        to:   &'static str,
    },
}

/// Solves Tower of Hanoi iteratively using a job pile.
/// Works like a robot reading job cards one at a time:
/// - If the card says "plan", break it into 3 new cards
/// - If the card says "execute", just do the ring move
/// No recursion — pure stack-based task processing!
///
/// * `n`      - number of rings to move
/// * `source` - pillar we're moving rings FROM
/// * `dest`   - pillar we're moving rings TO
/// * `spare`  - the extra pillar we can borrow
pub fn ring_machine(
    n: u64,
    source: &'static str,
    dest:   &'static str,
    spare:  &'static str,
) -> Vec<RingMove> {
    let mut journal: Vec<RingMove> = Vec::new();

    // 📋 The robot's job pile (last in = first done)
    let mut job_pile: Vec<JobCard> = Vec::new();

    // Load the first job card — the whole problem!
    job_pile.push(JobCard::PlanMove { n, source, dest, spare });

    // 🤖 Robot loop: keep working until no jobs left
    while let Some(card) = job_pile.pop() {
        match card {

            // ✅ Execute card — record the ring move
            JobCard::ExecuteMove { from, to } => {
                journal.push(RingMove { from, to });
            }

            // 🗂️ Plan card with 1 ring — just move it!
            JobCard::PlanMove { n: 1, source, dest, .. } => {
                journal.push(RingMove { from: source, to: dest });
            }

            // 🗂️ Plan card with n rings — split into 3 cards
            // Push in REVERSE order (pile is last-in-first-out)
            JobCard::PlanMove { n, source, dest, spare } => {
                // Card 3: stack n-1 rings spare → dest (done last)
                job_pile.push(JobCard::PlanMove {
                    n: n - 1,
                    source: spare,
                    dest,
                    spare: source,
                });

                // Card 2: move biggest ring source → dest
                job_pile.push(JobCard::ExecuteMove {
                    from: source,
                    to: dest,
                });

                // Card 1: clear n-1 rings source → spare (done first)
                job_pile.push(JobCard::PlanMove {
                    n: n - 1,
                    source,
                    dest: spare,
                    spare: dest,
                });
            }
        }
    }

    journal
}