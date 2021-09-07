//! [502. IPO](https://leetcode-cn.com/problems/ipo/)

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
struct Pair(i32, i32);

impl PartialOrd<Self> for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.0.cmp(&other.0) {
            Ordering::Equal => other.1.cmp(&self.1),
            not_equal => not_equal,
        }
    }
}

impl Solution {
    pub fn find_maximized_capital(k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut pairs: BinaryHeap<_> = profits
            .iter()
            .zip(capital.iter())
            .map(|(&profit, &capital)| Pair(profit, capital))
            .collect();

        let mut temp = vec![];
        let mut i = 0;
        while i < k {
            let mut found = false;
            while let Some(top) = pairs.pop() {
                if top.1 <= w {
                    pairs.extend(temp.drain(..));
                    w += top.0;
                    found = true;
                    break;
                } else {
                    temp.push(top);
                }
            }
            if !found {
                break;
            }
            i += 1;
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
