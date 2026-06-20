# Benchmark Results — Another Game

## Results (n=1000, all-even piles — worst case for early-exit)

| Algorithm | Time |
|---|---|
| `light_seeker` (Early-exit loop) | 241.88 ns |
| `light_merger` (Bitwise fold) | 62.64 ns |

## Interpretation

The bitwise fold approach is approximately **4x faster** than the
early-exit loop on this worst-case input.

`light_seeker` uses a `for` loop with an `if` branch checking
`pile % 2 == 1` on every element, with an early `return` only
possible when an odd pile is found. Since the benchmark input is
all-even (forcing the loop to scan every element), every iteration
pays the cost of a conditional branch.

`light_merger` instead computes `pile & 1` and folds all results
together with bitwise OR — a branch-free operation the CPU can
execute as a single fast instruction per element, with no
unpredictable branching to slow down the instruction pipeline.

This illustrates how branchless/bitwise techniques can outperform
branch-based code, especially when the branch is unpredictable or,
as in this worst case, never actually taken.
