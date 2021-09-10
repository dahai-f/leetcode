//! [1894. 找到需要补充粉笔的学生编号](https://leetcode-cn.com/problems/find-the-student-that-will-replace-the-chalk/)

use std::cmp::Ordering;

impl Solution {
    pub fn chalk_replacer(mut chalk: Vec<i32>, mut k: i32) -> i32 {
        let mut pre_sum = 0i32;
        for (i, x) in chalk.iter_mut().enumerate() {
            pre_sum = match pre_sum.checked_add(*x) {
                None => {
                    return i as i32;
                }
                Some(pre_sum) => {
                    if pre_sum > k {
                        return i as i32;
                    }
                    pre_sum
                }
            };
            *x = pre_sum;
        }
        k %= chalk.last().unwrap();
        let ans = chalk
            .binary_search_by(|&pre_sum| {
                if pre_sum <= k {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            })
            .unwrap_err();
        assert!(ans < chalk.len());
        ans as i32
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::chalk_replacer(vec![5, 1, 5], 22), 0);
    assert_eq!(Solution::chalk_replacer(vec![3, 4, 1, 2], 25), 1);
}
