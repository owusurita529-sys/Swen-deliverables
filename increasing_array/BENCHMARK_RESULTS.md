# Benchmark Results — Increasing Array

## Results (n=1000, worst-case decreasing array)

| Algorithm | Time |
|---|---|
| `lift_blocks` (Recursive) | 464.62 ns |

## Interpretation

This deliverable uses a single recursive algorithm benchmarked against
a worst-case fully-decreasing input of size 1000.

The recursive solution runs in well under a microsecond for n=1000,
confirming O(n) linear time complexity — each element is visited
exactly once via tail-recursive calls, with the running total and
floor height threaded through as accumulator parameters.

The very low absolute time (sub-microsecond) demonstrates that simple
linear-scan algorithms have minimal overhead when implemented with
tail-call-friendly recursion in Rust, which the compiler can often
optimize similarly to an iterative loop.
