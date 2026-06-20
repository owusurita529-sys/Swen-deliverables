# Benchmark Results — Tower of Hanoi

## Results (n=15 rings)

| Algorithm | Time |
|---|---|
| `ring_shifter` (Recursive) | 44.46 µs |
| `ring_machine` (Iterative with task pile) | 86.43 µs |

## Interpretation

The recursive approach is approximately **2x faster** than the
iterative task-pile approach.

`ring_shifter` maps directly onto the problem's natural recursive
structure — each call handles exactly one sub-problem and the
compiler can optimize the call chain efficiently with minimal
overhead per call.

`ring_machine` simulates recursion manually using a `Vec` as an
explicit stack of `Task` enum values. Each step requires allocating
and matching on enum variants, plus push/pop operations on the Vec,
which adds more overhead than a native function call.

This shows that for problems whose recursive structure is simple and
well-bounded (like Hanoi, which never exceeds O(2^n) calls but has
shallow depth O(n)), genuine recursion often beats a hand-rolled
iterative simulation of recursion.
