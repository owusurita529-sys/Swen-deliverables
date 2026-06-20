# Benchmark Results — Room Allocation

## Results (n=1000)

| Algorithm | Time |
|---|---|
| `hotel_desk` (BTreeMap / B-Tree) | 25.49 µs |
| `checkout_queue` (BinaryHeap) | 2.39 µs |

## Interpretation

The BinaryHeap approach is approximately **10x faster** than the
BTreeMap approach for this workload.

`hotel_desk` performs a B-Tree range query (`range(..checkin)`) on
every guest to locate the earliest available checkout. Although this
is O(log n) per operation, B-Tree range queries carry higher constant
overhead because they must traverse internal tree nodes and maintain
balance invariants.

`checkout_queue` only ever needs to know the single smallest checkout
time. A BinaryHeap supports `peek()` in O(1) and `push`/`pop` in
O(log n), which is exactly the access pattern this problem needs —
no full ordering of all elements is required, only repeated access to
the minimum. This narrower interface lets the heap outperform the
more general B-Tree structure.

This demonstrates that picking a data structure should be guided by
the actual access pattern: B-Trees excel at range queries and ordered
traversal, while heaps excel at repeated min/max extraction.
