// -------------------------------------------------------
// integration_test.rs
// Student: [RITA OWUSU]
// Tests for branch_counter (DFS) and queue_counter (BFS)
// Both must agree on every single test!
// -------------------------------------------------------

use subordinates::{branch_counter, queue_counter};

// 🧪 Helper: builds tree, runs BOTH approaches,
// checks they agree, returns the tally
fn both_agree(n: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    let mut org_chart: Vec<Vec<usize>> = vec![vec![]; n + 1];
    for &(boss, worker) in edges {
        org_chart[boss].push(worker);
        org_chart[worker].push(boss);
    }

    let mut dfs_tally = vec![0usize; n + 1];
    branch_counter(1, 0, &org_chart, &mut dfs_tally);

    let bfs_tally = queue_counter(n, &org_chart);

    for i in 1..=n {
        assert_eq!(
            dfs_tally[i], bfs_tally[i],
            "Approaches disagreed for employee {} with n={}",
            i, n
        );
    }
    dfs_tally
}

#[test]
fn test_official_example() {
    let tally = both_agree(5, &[(1,2),(1,3),(2,4),(3,5)]);
    assert_eq!(tally[1], 4);
    assert_eq!(tally[2], 1);
    assert_eq!(tally[3], 1);
    assert_eq!(tally[4], 0);
    assert_eq!(tally[5], 0);
}

#[test]
fn test_single_employee() {
    let org_chart: Vec<Vec<usize>> = vec![vec![]; 2];
    let mut tally = vec![0usize; 2];
    branch_counter(1, 0, &org_chart, &mut tally);
    assert_eq!(tally[1], 0);
    let bfs = queue_counter(1, &org_chart);
    assert_eq!(bfs[1], 0);
}

#[test]
fn test_straight_chain() {
    let tally = both_agree(5, &[(1,2),(2,3),(3,4),(4,5)]);
    assert_eq!(tally[1], 4);
    assert_eq!(tally[2], 3);
    assert_eq!(tally[3], 2);
    assert_eq!(tally[4], 1);
    assert_eq!(tally[5], 0);
}

#[test]
fn test_star_shape() {
    let tally = both_agree(5, &[(1,2),(1,3),(1,4),(1,5)]);
    assert_eq!(tally[1], 4);
    assert_eq!(tally[2], 0);
    assert_eq!(tally[3], 0);
    assert_eq!(tally[4], 0);
    assert_eq!(tally[5], 0);
}

#[test]
fn test_two_levels() {
    let tally = both_agree(6, &[(1,2),(1,3),(2,4),(2,5),(3,6)]);
    assert_eq!(tally[1], 5);
    assert_eq!(tally[2], 2);
    assert_eq!(tally[3], 1);
    assert_eq!(tally[4], 0);
    assert_eq!(tally[5], 0);
    assert_eq!(tally[6], 0);
}

#[test]
fn test_small_chain() {
    let tally = both_agree(3, &[(1,2),(2,3)]);
    assert_eq!(tally[1], 2);
    assert_eq!(tally[2], 1);
    assert_eq!(tally[3], 0);
}

#[test]
fn test_director_always_has_all() {
    let edges: Vec<(usize, usize)> = (1..10).map(|i| (i, i+1)).collect();
    let tally = both_agree(10, &edges);
    assert_eq!(tally[1], 9);
}

#[test]
fn test_leaves_have_zero() {
    let tally = both_agree(5, &[(1,2),(1,3),(2,4),(3,5)]);
    assert_eq!(tally[4], 0);
    assert_eq!(tally[5], 0);
}