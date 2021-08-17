//! [542. 01 矩阵](https://leetcode-cn.com/problems/01-matrix/)

use std::collections::LinkedList;

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let mut ans = vec![vec![i32::MAX; n]; m];
        let mut visited = vec![vec![false; n]; m];
        let mut q = LinkedList::new();
        for (row_i, row) in mat.iter().enumerate() {
            for (col_i, &x) in row.iter().enumerate() {
                if x == 0 {
                    ans[row_i][col_i] = 0;
                    visited[row_i][col_i] = true;
                    q.push_back((row_i, col_i));
                }
            }
        }

        while let Some((row, col)) = q.pop_front() {
            let distance = ans[row][col] + 1;
            Self::scan(
                row,
                col.wrapping_sub(1),
                distance,
                &mut visited,
                &mut ans,
                &mut q,
            );
            Self::scan(row, col + 1, distance, &mut visited, &mut ans, &mut q);
            Self::scan(
                row.wrapping_sub(1),
                col,
                distance,
                &mut visited,
                &mut ans,
                &mut q,
            );
            Self::scan(row + 1, col, distance, &mut visited, &mut ans, &mut q);
        }
        ans
    }

    fn scan(
        row: usize,
        col: usize,
        distance: i32,
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
        ans[row][col] = distance;
        q.push_back((row, col));
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]),
        vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]
    );
    assert_eq!(
        Solution::update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]),
        vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]]
    );
    assert_eq!(
        Solution::update_matrix(vec![
            vec![1, 0, 1, 1, 0, 0, 1, 0, 0, 1],
            vec![0, 1, 1, 0, 1, 0, 1, 0, 1, 1],
            vec![0, 0, 1, 0, 1, 0, 0, 1, 0, 0],
            vec![1, 0, 1, 0, 1, 1, 1, 1, 1, 1],
            vec![0, 1, 0, 1, 1, 0, 0, 0, 0, 1],
            vec![0, 0, 1, 0, 1, 1, 1, 0, 1, 0],
            vec![0, 1, 0, 1, 0, 1, 0, 0, 1, 1],
            vec![1, 0, 0, 0, 1, 1, 1, 1, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 0, 1, 0],
            vec![1, 1, 1, 1, 0, 1, 0, 0, 1, 1]
        ]),
        vec![
            vec![1, 0, 1, 1, 0, 0, 1, 0, 0, 1],
            vec![0, 1, 1, 0, 1, 0, 1, 0, 1, 1],
            vec![0, 0, 1, 0, 1, 0, 0, 1, 0, 0],
            vec![1, 0, 1, 0, 1, 1, 1, 1, 1, 1],
            vec![0, 1, 0, 1, 1, 0, 0, 0, 0, 1],
            vec![0, 0, 1, 0, 1, 1, 1, 0, 1, 0],
            vec![0, 1, 0, 1, 0, 1, 0, 0, 1, 1],
            vec![1, 0, 0, 0, 1, 2, 1, 1, 0, 1],
            vec![2, 1, 1, 1, 1, 2, 1, 0, 1, 0],
            vec![3, 2, 2, 1, 0, 1, 0, 0, 1, 1]
        ]
    );
}
