//! [120. 三角形最小路径和](https://leetcode-cn.com/problems/triangle/)
impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();
        let mut triangle = triangle.iter();
        let first = triangle.next().unwrap();
        let mut min = Vec::with_capacity(n);
        for i in 0..n {
            min.push(first.get(i).copied().unwrap_or(i32::MAX));
        }
        for row in triangle {
            for (col_i, x) in row.iter().enumerate().rev() {
                min[col_i] = x + if col_i == 0 {
                    min[col_i]
                } else {
                    min[col_i - 1].min(min[col_i])
                };
            }
        }

        min.into_iter().min().unwrap()
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
        11
    );
    assert_eq!(Solution::minimum_total(vec![vec![-10]]), -10);
}
