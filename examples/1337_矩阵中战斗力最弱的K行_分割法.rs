//! [1337. 矩阵中战斗力最弱的 K 行](https://leetcode-cn.com/problems/the-k-weakest-rows-in-a-matrix/)

use std::cmp::Ordering;

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut num_row_pairs: Vec<(usize, i32)> = mat
            .iter()
            .enumerate()
            .map(|(row_i, row)| {
                let num = row
                    .binary_search_by(|x| {
                        if *x == 1 {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    })
                    .unwrap_or_else(|i| i);
                (num, row_i as i32)
            })
            .collect();

        Self::partition_at(&mut num_row_pairs, k);
        num_row_pairs[..k].sort_unstable();
        num_row_pairs[..k]
            .iter()
            .map(|(_num, row_i)| *row_i)
            .collect()
    }

    fn partition_at(vec: &mut [(usize, i32)], index: usize) {
        let mut left = 0;
        let mut right = vec.len().wrapping_sub(1);
        while left < right && right < vec.len() {
            while left < right && vec[left] <= vec[right] {
                left += 1;
            }
            if left < right {
                vec.swap(left, right);
            }
            while left < right && vec[left] <= vec[right] {
                right = right.wrapping_sub(1);
            }
            if left < right && right < vec.len() {
                vec.swap(left, right);
            }
        }

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
        Solution::k_weakest_rows(
            vec![
                vec![1, 1, 0],
                vec![1, 0, 0],
                vec![1, 0, 0],
                vec![1, 1, 1],
                vec![1, 1, 0],
                vec![0, 0, 0],
            ],
            4,
        ),
        vec![5, 1, 2, 0]
    );
    assert_eq!(
        Solution::k_weakest_rows(vec![vec![1, 0], vec![1, 0], vec![1, 0], vec![1, 1]], 4),
        vec![0, 1, 2, 3]
    );
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
