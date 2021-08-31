//! [528. 按权重随机选择](https://leetcode-cn.com/problems/random-pick-with-weight/)

use rand::distributions::{Distribution, Uniform};
use std::cmp::Ordering;

struct Solution {
    pre_sum: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(mut w: Vec<i32>) -> Self {
        for i in 1..w.len() {
            w[i] += w[i - 1];
        }
        Self { pre_sum: w }
    }

    fn pick_index(&self) -> i32 {
        let &sum = self.pre_sum.last().unwrap();
        let range = Uniform::from(1..=sum);
        let mut rng = rand::thread_rng();
        let num = range.sample(&mut rng);
        self.pre_sum
            .binary_search_by(|&pre_sum| {
                if pre_sum < num {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            })
            .unwrap_or_else(|e| e) as i32
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(w);
 * let ret_1: i32 = obj.pick_index();
 */

fn main() {}
