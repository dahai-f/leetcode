//! [1337. 矩阵中战斗力最弱的 K 行](https://leetcode-cn.com/problems/the-k-weakest-rows-in-a-matrix/)

use std::cmp::Ordering::{Greater, Less};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut heap: BinaryHeap<Reverse<(usize, i32)>> = mat
            .iter()
            .enumerate()
            .map(|(row_i, row)| {
                let num = row
                    .binary_search_by(|x| if *x == 1 { Less } else { Greater })
                    .unwrap_or_else(|i| i);
                Reverse((num, row_i as i32))
            })
            .collect();
        let mut res = Vec::with_capacity(k);
        for _i in 0..k {
            res.push(heap.pop().unwrap().0 .1);
        }
        res
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::k_weakest_rows(
            vec![
                vec![1, 1, 0, 0, 0],
                vec![1, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 0],
                vec![1, 1, 0, 0, 0],
                vec![1, 1, 1, 1, 1],
            ],
            3,
        ),
        vec![2, 0, 3]
    );
    assert_eq!(
        Solution::k_weakest_rows(
            vec![
                vec![1, 0, 0, 0],
                vec![1, 1, 1, 1],
                vec![1, 0, 0, 0],
                vec![1, 0, 0, 0],
            ],
            2,
        ),
        vec![0, 2]
    );
}
