# Benchmark Results — Movie Festival

## Results (n=1000, heavily overlapping screenings)

| Algorithm | Time |
|---|---|
| `screen_scheduler` (Sort + for-loop) | 884.82 ns |
| `film_sweeper` (Sort + while-loop with index) | 560.49 ns |

## Interpretation

The while-loop/index-based approach is approximately **1.6x faster**
than the for-loop/iterator approach.

Both algorithms use the identical greedy heuristic — sort by finish
time, then pick any film that starts after the screen is free — so
the only difference is mechanical: `screen_scheduler` consumes an
owned `Vec` via a `for` loop (iterator-based), while `film_sweeper`
indexes into the vector directly via an explicit `idx` counter in a
`while` loop.

The explicit index version allows the compiler to generate tighter
bounds-checked array access without the overhead of constructing and
advancing an iterator object, leading to the modest but measurable
speed difference observed.

This demonstrates that even when two algorithms are asymptotically
identical (both O(n log n) due to the sort) and use the same
heuristic, low-level implementation choices still produce measurable
performance differences.
