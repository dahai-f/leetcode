//! [面试题 17.14. 最小K个数](https://leetcode-cn.com/problems/smallest-k-lcci/)
use rand::prelude::*;
use std::cmp::Ordering;

impl Solution {
    pub fn smallest_k(mut arr: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        Self::partition_at(&mut arr, k - 1);
        unsafe {
            arr.set_len(k);
        }
        arr
    }

    fn partition_at(vec: &mut [i32], index: usize) {
        if vec.len() <= 1 {
            return;
        }
        let mut rng = rand::thread_rng();
        let pivot_i = rng.gen_range(1..vec.len());
        vec.swap(0, pivot_i);
        let pivot = vec[0];
        let mut left = 0;
        for i in 1..vec.len() {
            if vec[i] < pivot {
                left += 1;
                vec.swap(i, left);
            }
        }
        vec.swap(0, left);

        match left.cmp(&index) {
            Ordering::Less => Self::partition_at(&mut vec[(left + 1)..], index - left - 1),
            Ordering::Equal => {}
            Ordering::Greater => Self::partition_at(&mut vec[..left], index),
        }
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::smallest_k(vec![1, 3, 5, 7, 2, 4, 6, 8], 4),
        vec![1, 2, 3, 4]
    );
}
