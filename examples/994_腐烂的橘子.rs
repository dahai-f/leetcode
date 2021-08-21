//! [994. 腐烂的橘子](https://leetcode-cn.com/problems/rotting-oranges/)

use std::collections::LinkedList;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        let mut distance = vec![vec![i32::MAX; grid[0].len()]; grid.len()];
        let mut q = LinkedList::new();
        for (row_i, row) in grid.iter().enumerate() {
            for (col_i, &x) in row.iter().enumerate() {
                if x == 0 {
                    visited[row_i][col_i] = true;
                    distance[row_i][col_i] = 0;
                } else if x == 2 {
                    visited[row_i][col_i] = true;
                    distance[row_i][col_i] = 0;
                    q.push_back((row_i, col_i));
                }
            }
        }

        while let Some((row, col)) = q.pop_front() {
            let new_distance = distance[row][col] + 1;
            Self::scan(
                row,
                col.wrapping_sub(1),
                new_distance,
                &mut visited,
                &mut distance,
                &mut q,
            );
            Self::scan(
                row,
                col.saturating_add(1),
                new_distance,
                &mut visited,
                &mut distance,
                &mut q,
            );
            Self::scan(
                row.wrapping_sub(1),
                col,
                new_distance,
                &mut visited,
                &mut distance,
                &mut q,
            );
            Self::scan(
                row.saturating_add(1),
                col,
                new_distance,
                &mut visited,
                &mut distance,
                &mut q,
            );
        }

        let time = distance
            .iter()
            .map(|distance| distance.iter().copied().max().unwrap_or(i32::MAX))
            .max()
            .unwrap_or(i32::MAX);
        if time == i32::MAX {
            -1
        } else {
            time
        }
    }

    fn scan(
        row: usize,
        col: usize,
        new_distance: i32,
        visited: &mut Vec<Vec<bool>>,
        ans: &mut Vec<Vec<i32>>,
        q: &mut LinkedList<(usize, usize)>,
    ) {
        if row >= visited.len() || col >= visited[0].len() {
            return;
        }
        if visited[row][col] {
            return;
        }
        visited[row][col] = true;

        ans[row][col] = new_distance;
        q.push_back((row, col));
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]]),
        4
    );
    assert_eq!(
        Solution::oranges_rotting(vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]]),
        -1
    );
    assert_eq!(Solution::oranges_rotting(vec![vec![0, 2]]), 0);
}
