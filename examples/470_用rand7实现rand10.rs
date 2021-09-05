//! [470. 用 Rand7() 实现 Rand10()](https://leetcode-cn.com/problems/implement-rand10-using-rand7/)

/**
 * The rand7() API is already defined for you
 * @return a random integer in the range 1 to
 * fn rand7() -> i32
 */

impl Solution {
    pub fn rand10() -> i32 {
        loop {
            let row = rand7();
            let col = rand7();
            let idx = (row - 1) * 7 + col;
            if idx <= 40 {
                return (idx - 1) % 10 + 1;
            }
        }
    }
}

fn rand7() -> i32 {
    use rand::distributions::Uniform;
    use rand::prelude::*;

    thread_local! {
        static RANGE: Uniform<i32> = Uniform::from(1..=7);
    }
    let mut rng = rand::thread_rng();
    RANGE.with(|range| range.sample(&mut rng))
}

struct Solution;

fn main() {
    Solution::rand10();
}
