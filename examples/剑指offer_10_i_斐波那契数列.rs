//! [剑指 Offer 10- I. 斐波那契数列](https://leetcode-cn.com/problems/fei-bo-na-qi-shu-lie-lcof/)

impl Solution {
    pub fn fib(mut n: i32) -> i32 {
        if n == 0 {
            0
        } else {
            let mut p = 1;
            let mut pp = 0;
            while n > 1 {
                let cur = (pp + p) % 1000000007;
                pp = p;
                p = cur;
                n -= 1;
            }
            p
        }
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
    assert_eq!(Solution::fib(76), 598991529);
}
