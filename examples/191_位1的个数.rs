//! [191. 位1的个数](https://leetcode-cn.com/problems/number-of-1-bits/)

impl Solution {
    pub fn hammingWeight(mut n: u32) -> i32 {
        let mut count = 0;
        while n > 0 {
            n &= n - 1;
            count += 1;
        }
        count
    }
}

struct Solution;

fn main() {}
