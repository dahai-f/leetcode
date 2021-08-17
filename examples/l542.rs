//! [542. 01 矩阵](https://leetcode-cn.com/problems/01-matrix/)

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let mut ans = vec![vec![-1; n]; m];
        for row in 0..m {
            for col in 0..n {
                Self::min_distance(&mat, row, col, &mut ans);
            }
        }
        ans
    }

    fn min_distance(mat: &[Vec<i32>], row: usize, col: usize, ans: &mut Vec<Vec<i32>>) -> i32 {
        if row >= mat.len() || col >= mat[0].len() {
            i32::MAX
        } else {
            if ans[row][col] != i32::MAX {
                if mat[row][col] == 1 {
                    ans[row][col] = i32::MAX;
                    ans[row][col] = 1_i32.saturating_add(
                        Self::min_distance(mat, row, col.wrapping_sub(1), ans)
                            .min(Self::min_distance(mat, row, col + 1, ans))
                            .min(Self::min_distance(mat, row.wrapping_sub(1), col, ans))
                            .min(Self::min_distance(mat, row + 1, col, ans)),
                    );
                } else {
                    ans[row][col] = 0;
                }
            }
            ans[row][col]
        }
    }
}

struct Solution;

fn main() {
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
    assert_eq!(
        Solution::update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]),
        vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]
    );
    assert_eq!(
        Solution::update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]),
        vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]]
    );
}
