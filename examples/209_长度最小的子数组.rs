//! [209. 长度最小的子数组](https://leetcode-cn.com/problems/minimum-size-subarray-sum/)

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut min_len = i32::MAX;
        let mut left = 0;
        let mut right = 0;

        if min_len == i32::MAX {
            0
        } else {
            min_len
        }
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
    assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
    assert_eq!(
        Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]),
        0
    );
}
