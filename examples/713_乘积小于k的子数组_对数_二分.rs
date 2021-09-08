//! [713. 乘积小于K的子数组](https://leetcode-cn.com/problems/subarray-product-less-than-k/)

use std::cmp::Ordering;

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut count = 0;
        let n = nums.len();
        let mut nums = nums.iter().map(|&num| (num as f64).log10());
        let mut prefix = Vec::with_capacity(nums.len());
        prefix.push(nums.next().unwrap());
        for (pre_i, p) in nums.enumerate() {
            prefix.push(prefix[pre_i] + p)
        }

        let log_k = (k as f64).log10();

        for i in 0..n {
            let pre = *prefix.get(i.wrapping_sub(1)).unwrap_or(&0f64);
            let target = pre + log_k - 1e-9;
            let j = prefix[i..]
                .binary_search_by(|&p| {
                    if p < target {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                })
                .unwrap_err();
            count += j;
        }

        count as i32
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::num_subarray_product_less_than_k(
            vec![10, 3, 3, 7, 2, 9, 7, 4, 7, 2, 8, 6, 5, 1, 5],
            30
        ),
        26
    );
    assert_eq!(
        Solution::num_subarray_product_less_than_k(
            vec![10, 9, 10, 4, 3, 8, 3, 3, 6, 2, 10, 10, 9, 3],
            19
        ),
        18
    );
    assert_eq!(
        Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100),
        8
    );
    assert_eq!(
        Solution::num_subarray_product_less_than_k(vec![1, 2, 3], 0),
        0
    );
}
