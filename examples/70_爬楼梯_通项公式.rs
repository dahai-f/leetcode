//! [70. 爬楼梯](https://leetcode-cn.com/problems/climbing-stairs/)

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let n = n + 1;
        let sqrt_5 = 5f64.sqrt();
        ((((1f64 + sqrt_5) / 2f64).powi(n) - ((1f64 - sqrt_5) / 2f64).powi(n)) / sqrt_5).round()
            as i32
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::climb_stairs(1), 1);
    assert_eq!(Solution::climb_stairs(2), 2);
    assert_eq!(Solution::climb_stairs(3), 3);
}
