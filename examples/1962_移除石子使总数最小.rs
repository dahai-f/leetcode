//! [1962. 移除石子使总数最小](https://leetcode-cn.com/problems/remove-stones-to-minimize-the-total/)
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        let mut piles = BinaryHeap::from(piles);
        let mut i = 0;
        while i < k {
            let max = {
                match piles.pop() {
                    None => {
                        break;
                    }
                    Some(max) => max,
                }
            };
            let new = max - max / 2;
            piles.push(new);
            if new == max {
                break;
            }
            i += 1;
        }
        piles.iter().sum()
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::min_stone_sum(vec![5, 4, 9], 2), 12);
    assert_eq!(Solution::min_stone_sum(vec![4, 3, 6, 7], 3), 12);
}
