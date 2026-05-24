use std::io::{self, BufWriter, Write};
use std::time::Instant;

fn hanoi_recursive(count: u64, from: u8, to: u8, via: u8, out: &mut impl Write) {
    if count == 0 {
        return;
    }
    hanoi_recursive(count - 1, from, via, to, out);
    writeln!(out, "{} {}", from, to).unwrap();
    hanoi_recursive(count - 1, via, to, from, out);
}

fn hanoi_iterative(count: u64, from: u8, to: u8, via: u8, out: &mut impl Write) {
    let mut stack: Vec<(u64, u8, u8, u8)> = vec![(count, from, to, via)];
    while let Some((n, src, dst, hlp)) = stack.pop() {
        if n == 0 {
            continue;
        } else if n == 1 {
            writeln!(out, "{} {}", src, dst).unwrap();
        } else {
            stack.push((n - 1, hlp, dst, src));
            stack.push((1,     src, dst, hlp));
            stack.push((n - 1, src, hlp, dst));
        }
    }
}

fn main() {
    let mut raw = String::new();
    io::stdin().read_line(&mut raw).unwrap();
    let n: u64 = raw.trim().parse().unwrap();
    let total: u64 = (1 << n) - 1;

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    writeln!(out, "{}", total).unwrap();

    // --- Benchmark recursive ---
    let t1 = Instant::now();
    let mut buf1: Vec<u8> = Vec::new();
    hanoi_recursive(n, 1, 3, 2, &mut buf1);
    let rec_time = t1.elapsed();

    // --- Benchmark iterative ---
    let t2 = Instant::now();
    let mut buf2: Vec<u8> = Vec::new();
    hanoi_iterative(n, 1, 3, 2, &mut buf2);
    let itr_time = t2.elapsed();

    // Print the actual moves (recursive output used for submission)
    out.write_all(&buf1).unwrap();

    // Print timing to stderr — won't affect CSES judge output
    eprintln!("Recursive : {:?}", rec_time);
    eprintln!("Iterative : {:?}", itr_time);
}