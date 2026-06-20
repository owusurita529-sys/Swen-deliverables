# Benchmark Results — Two Sets

## Results (n=1000)

| Algorithm | Time |
|---|---|
| `tray_filler` (Greedy top-down) | 1.2523 µs |
| `tray_builder` (Mathematical pairs) | 950.45 ns |

## Interpretation

The mathematical pairing approach is roughly **24% faster** than the
greedy approach.

`tray_filler` counts down from n, checking each number against a
running "still needed" threshold — a single linear pass with a
branch per element.

`tray_builder` uses two pointers walking inward from both ends of the
range simultaneously, pairing numbers that sum to n+1. This requires
fewer iterations overall since each loop iteration can place up to
two numbers at once.

Both algorithms are O(n), so the difference comes down to constant
factors: the two-pointer approach does less branching per step since
it only needs to decide which side to push the pair to, rather than
checking a continuously shrinking remaining-capacity value.
