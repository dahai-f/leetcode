//! [231. 2 的幂](https://leetcode-cn.com/problems/power-of-two/)

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        // n > 0 && (n & (n - 1)) == 0
        n > 0 && (n & (-n)) == n
    }
}

struct Solution;

fn main() {}
