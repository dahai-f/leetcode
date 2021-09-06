//! [74. 搜索二维矩阵](https://leetcode-cn.com/problems/search-a-2d-matrix/)
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        match matrix.binary_search_by_key(&target, |row| *row.last().unwrap()) {
            Ok(_found) => true,
            Err(not_found) => matrix
                .get(not_found)
                .and_then(|row| row.binary_search(&target).ok())
                .is_some(),
        }
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
