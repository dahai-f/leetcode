//! [528. 按权重随机选择](https://leetcode-cn.com/problems/random-pick-with-weight/)

use rand::distributions::{Distribution, Uniform};
use rand::prelude::ThreadRng;
use std::cmp::Ordering;

struct Solution {
    pre_sum: Vec<i32>,
    uniform_range: Uniform<i32>,
    rng: ThreadRng,
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
        let &sum = w.last().unwrap();
        Self {
            pre_sum: w,
            uniform_range: (Uniform::from(1..=sum)),
            rng: Default::default(),
        }
    }

    fn pick_index(&mut self) -> i32 {
        let num = self.uniform_range.sample(&mut self.rng);
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

fn validate(vec: Vec<i32>, mut solution: Solution, pick_count: usize) -> bool {
    let mut count = vec![0; vec.len()];
    for _i in 0..pick_count {
        count[solution.pick_index() as usize] += 1;
    }
    let mut vec_with_i: Vec<_> = vec.iter().copied().enumerate().collect();
    let mut count_with_i: Vec<_> = count.iter().copied().enumerate().collect();
    vec_with_i.sort_unstable_by_key(|(_i, num)| *num);
    count_with_i.sort_unstable_by_key(|(_i, num)| *num);
    vec_with_i
        .iter()
        .zip(count_with_i.iter())
        .all(|((vec_i, _vec_num), (count_i, _count_num))| vec_i == count_i)
}

fn main() {
    let vec = vec![1];
    let solution = Solution::new(vec.clone());
    assert!(validate(vec, solution, 1));
    let vec = vec![1, 3];
    let solution = Solution::new(vec.clone());
    assert!(validate(vec, solution, 5));
}
