use std::cmp::Ordering;

/// [167. 两数之和 II - 输入有序数组](https://leetcode-cn.com/problems/two-sum-ii-input-array-is-sorted/)

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut right = numbers.len().wrapping_sub(1);
        let mut left = 0;
        while left < right {
            let sum = numbers[left] + numbers[right];
            match sum.cmp(&target) {
                Ordering::Less => {
                    left += 1;
                }
                Ordering::Equal => {
                    return vec![left as i32 + 1, right as i32 + 1];
                }
                Ordering::Greater => {
                    right = right.wrapping_sub(1);
                }
            }
        }
        vec![]
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
    assert_eq!(Solution::two_sum(vec![-1, 0], -1), vec![1, 2]);
}
