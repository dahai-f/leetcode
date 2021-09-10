//! [1894. 找到需要补充粉笔的学生编号](https://leetcode-cn.com/problems/find-the-student-that-will-replace-the-chalk/)

use std::cmp::Ordering;

impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let mut pre_sum = Vec::with_capacity(chalk.len());
        chalk.iter().fold(0u64, |total, cur| {
            let new_total = total.checked_add(*cur as u64).unwrap();
            pre_sum.push(new_total);
            new_total
        });
        let k = k as u64 % pre_sum.last().unwrap();
        let ans = pre_sum
            .binary_search_by(|&pre_sum| {
                if pre_sum <= k {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            })
            .unwrap_err();
        assert!(ans < pre_sum.len());
        ans as i32
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::chalk_replacer(vec![5, 1, 5], 22), 0);
    assert_eq!(Solution::chalk_replacer(vec![3, 4, 1, 2], 25), 1);
}
