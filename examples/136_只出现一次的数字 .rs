//! [136. 只出现一次的数字](https://leetcode-cn.com/problems/single-number/)

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut num = 0;
        for x in nums {
            num ^= x;
        }
        num
    }
}

struct Solution;

fn main() {}
