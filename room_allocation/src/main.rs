// -------------------------------------------------------
// main.rs
// Student: [RITA OWUSU]
// Room Allocation — CSES compatible, self-contained
// -------------------------------------------------------

use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::io::{self, BufRead};

fn checkout_queue(bookings: &[(i64, i64)]) -> (usize, Vec<usize>) {
    let mut guest_list: Vec<(i64, i64, usize)> = bookings
        .iter()
        .enumerate()
        .map(|(i, &(a, d))| (a, d, i))
        .collect();
    guest_list.sort();

    let mut checkout_heap: BinaryHeap<Reverse<(i64, usize)>> =
        BinaryHeap::new();
    let mut room_log = vec![0usize; bookings.len()];
    let mut room_tally = 1usize;

    for (checkin, checkout, guest_id) in guest_list {
        let assigned = if let Some(&Reverse((earliest, room))) =
            checkout_heap.peek()
        {
            if earliest <= checkin {
                checkout_heap.pop();
                room
            } else {
                let r = room_tally;
                room_tally += 1;
                r
            }
        } else {
            let r = room_tally;
            room_tally += 1;
            r
        };

        room_log[guest_id] = assigned;
        checkout_heap.push(Reverse((checkout, assigned)));
    }

    (room_tally - 1, room_log)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines
        .next().unwrap().unwrap()
        .trim()
        .parse()
        .unwrap();

    let mut bookings: Vec<(i64, i64)> = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.trim().split_whitespace();
        let checkin: i64 = parts.next().unwrap().parse().unwrap();
        let checkout: i64 = parts.next().unwrap().parse().unwrap();
        bookings.push((checkin, checkout));
    }

    let (rooms_needed, room_log) = checkout_queue(&bookings);

    println!("{}", rooms_needed);
    for room in &room_log {
        println!("{}", room);
    }
}