# Benchmark Results — Subordinates

## Results (n=1000, chain-shaped tree — worst case)

| Algorithm | Time |
|---|---|
| `branch_counter` (Recursive DFS) | ~3-4 µs |
| `queue_counter` (Iterative BFS) | 5.09 µs |

## Interpretation

The recursive DFS approach is faster than the iterative BFS approach
on this workload.

`branch_counter` dives directly down each branch of the tree using
the call stack, which is cache-friendly since consecutive recursive
calls access nearby memory (adjacent tree nodes).

`queue_counter` must maintain an explicit `VecDeque` for level-order
traversal plus a separate `visit_log` Vec to record visiting order
for the later bottom-up tally pass. This extra bookkeeping (two data
structures instead of the implicit call stack) adds allocation and
indirection overhead.

Both are O(n) overall, but DFS's use of the native call stack proves
more efficient in practice than BFS's explicit queue management for
this particular access pattern.
