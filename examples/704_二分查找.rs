//! [704. 二分查找](https://leetcode-cn.com/problems/binary-search/)

use std::cmp::Ordering;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();
        while left < right {
            let mid = (left + right) / 2;
            match nums[mid].cmp(&target) {
                Ordering::Less => {
                    left = mid + 1;
                }
                Ordering::Equal => {
                    return mid as i32;
                }
                Ordering::Greater => {
                    right = mid;
                }
            }
        }
        -1
    }
}

struct Solution;

fn main() {
    let res = Solution::search(vec![-1, 0, 3, 5, 9, 12], 9);
    println!("{}", res);
}
