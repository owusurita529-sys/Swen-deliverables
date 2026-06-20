# Benchmark Results — Counting Rooms

## Results (100x100 checkerboard grid — many small rooms)

| Algorithm | Time |
|---|---|
| `count_with_diver` (Recursive DFS flood fill) | 26.21 µs |
| `count_with_sweeper` (Iterative BFS flood fill) | 97.16 µs |

## Interpretation

The recursive DFS flood fill is approximately **3.7x faster** than
the BFS flood fill on this checkerboard test case.

`count_with_diver` dives deep into one connected region before
backtracking, which keeps memory access localized — important for
CPU cache performance.

`count_with_sweeper` maintains an explicit `VecDeque` "wave" that
grows and shrinks as it explores each room, with frequent push/pop
operations. On a checkerboard with hundreds of single-tile rooms,
this queue management overhead is paid hundreds of times, each for a
trivially small region, making the constant-factor cost dominate.

This highlights that on graphs with many small connected components,
recursive DFS's lower per-call overhead can outweigh BFS's typically
similar asymptotic complexity (both are O(rows × cols)).
