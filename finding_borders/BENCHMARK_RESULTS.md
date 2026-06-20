# Benchmark Results — Finding Borders

## Results (n=1000, string of repeated characters — worst case)

| Algorithm | Time |
|---|---|
| `ribbon_tracer` (KMP failure function) | 2.15 µs |
| `ribbon_checker` (Naive brute force) | 10.56 µs |

## Interpretation

The KMP approach is approximately **5x faster** than the naive
approach.

`ribbon_tracer` builds a failure/jump table in a single O(n) pass,
where each character is examined a small, bounded number of times
(amortized O(1) due to the way the `match_len` pointer only moves
forward overall).

`ribbon_checker` checks every possible border length explicitly,
comparing a prefix slice against a suffix slice for each candidate
length. Each comparison itself can take up to O(n) time, giving
O(n²) worst-case behaviour — clearly visible on this worst-case
all-same-character input designed to maximize comparison work.

This is a textbook demonstration of why KMP's amortized linear-time
guarantee matters: as n grows, the gap between O(n) and O(n²)
widens dramatically.
