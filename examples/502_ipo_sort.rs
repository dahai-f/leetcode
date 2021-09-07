//! [502. IPO](https://leetcode-cn.com/problems/ipo/)

use std::collections::BinaryHeap;

impl Solution {
    pub fn find_maximized_capital(k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::new();
        let mut sorted_by_capital: Vec<_> = capital
            .iter()
            .copied()
            .zip(profits.iter().copied())
            .collect();
        sorted_by_capital.sort_unstable_by_key(|(capital, _profit)| *capital);
        let mut cur = 0;
        for _i in 0..k {
            while cur < sorted_by_capital.len() && sorted_by_capital[cur].0 <= w {
                heap.push(sorted_by_capital[cur].1);
                cur += 1;
            }
            if let Some(top) = heap.pop() {
                w += top;
            } else {
                break;
            }
        }
        w
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::find_maximized_capital(2, 0, vec![1, 2, 3], vec![0, 1, 1]),
        4
    );
    assert_eq!(
        Solution::find_maximized_capital(3, 0, vec![1, 2, 3], vec![0, 1, 2]),
        6
    );
}
