//! [1588. 所有奇数长度子数组的和](https://leetcode-cn.com/problems/sum-of-all-odd-length-subarrays/)

impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let mut sum = 0;
        for (i, &x) in arr.iter().enumerate() {
            let left_count = i;
            let right_count = arr.len() - i - 1;
            let left_odd = (left_count + 1) / 2;
            let right_odd = (right_count + 1) / 2;
            let left_even = left_count / 2 + 1;
            let right_even = right_count / 2 + 1;
            sum += x * (left_odd * right_odd + left_even * right_even) as i32;
        }
        sum
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::sum_odd_length_subarrays(vec![1, 4, 2, 5, 3]), 58);
    assert_eq!(Solution::sum_odd_length_subarrays(vec![1, 2]), 3);
    assert_eq!(Solution::sum_odd_length_subarrays(vec![10, 11, 12]), 66);
}
