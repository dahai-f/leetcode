//! [1588. 所有奇数长度子数组的和](https://leetcode-cn.com/problems/sum-of-all-odd-length-subarrays/)

impl Solution {
    pub fn sum_odd_length_subarrays(mut arr: Vec<i32>) -> i32 {
        for i in 1..arr.len() {
            arr[i] += arr[i - 1];
        }

        let mut sum = 0;
        let mut len = 1;
        while len <= arr.len() {
            sum += arr[len - 1];
            for x in len..arr.len() {
                sum += arr[x] - arr[x - len];
            }
            len += 2;
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
