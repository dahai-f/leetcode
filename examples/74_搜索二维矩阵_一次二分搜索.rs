//! [74. 搜索二维矩阵](https://leetcode-cn.com/problems/search-a-2d-matrix/)
use std::cmp::Ordering;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut right = m * n;
        let mut left = 0;
        while left < right {
            let mid = left + (right - left) / 2;
            let mid_row = mid / n;
            let mid_col = mid % n;
            match matrix[mid_row][mid_col].cmp(&target) {
                Ordering::Less => {
                    left = mid + 1;
                }
                Ordering::Equal => {
                    return true;
                }
                Ordering::Greater => {
                    right = mid;
                }
            }
        }
        false
    }
}

struct Solution;

fn main() {
    assert!(Solution::search_matrix(
        vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
        3
    ));
    assert!(!Solution::search_matrix(
        vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
        13
    ));
}
