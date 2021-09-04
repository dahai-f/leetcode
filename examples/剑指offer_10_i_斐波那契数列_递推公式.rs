//! [剑指 Offer 10- I. 斐波那契数列](https://leetcode-cn.com/problems/fei-bo-na-qi-shu-lie-lcof/)

impl Solution {
    pub fn fib(n: i32) -> i32 {
        let sqrt_5 = 5f64.sqrt();
        ((1f64 / sqrt_5 * (((1f64 + sqrt_5) / 2f64).powi(n) - ((1f64 - sqrt_5) / 2f64).powi(n)))
            .round() as u64
            % 1000000007) as i32
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::fib(0), 0);
    assert_eq!(Solution::fib(1), 1);
    assert_eq!(Solution::fib(2), 1);
    assert_eq!(Solution::fib(5), 5);
    assert_eq!(Solution::fib(32), 2178309);
    assert_eq!(Solution::fib(45), 134903163);
    // n 达到76时开始发生误差导致的错误
    assert_eq!(Solution::fib(76), 598991529);
}
