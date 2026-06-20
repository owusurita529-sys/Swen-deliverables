# Benchmark Results — Josephus Problem I

## Results (n=1000)

| Algorithm | Time |
|---|---|
| `ring_counter` (BTreeSet simulation) | 174.67 µs |
| `champion_finder` (Mathematical formula) | 4.64 µs |

## Interpretation

The mathematical formula approach is approximately **38x faster** than
the BTreeSet simulation.

`ring_counter` simulates every single elimination step. Each step
requires a BST lookup costing O(log n), and we perform n such steps,
giving total complexity O(n log n).

`champion_finder` uses the closed-form Josephus recurrence
`C(n) = (C(n-1) + 2) % n`, computing the winner directly with a single
multiplication and modulo per recursive call — O(n) total, with much
lower constant overhead since there is no tree balancing or pointer
chasing involved.

This demonstrates a core algorithmic lesson: when a mathematical
closed-form exists for a problem, it will typically outperform any
general-purpose data-structure-based simulation, since it avoids the
overhead of maintaining structure that the simulation doesn't
strictly need.
