//! [209. 长度最小的子数组](https://leetcode-cn.com/problems/minimum-size-subarray-sum/)

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut min_len = usize::MAX;
        let mut left = 0;
        let mut right = 0;
        let mut sum = 0;
        while right < nums.len() {
            sum += nums[right];
            right += 1;
            if sum >= target {
                while sum >= target && left < nums.len() {
                    sum -= nums[left];
                    left += 1;
                }
                let len = right - left + 1;
                if len < min_len {
                    min_len = len;
                }
            }
        }

        if min_len == usize::MAX {
            0
        } else {
            min_len as i32
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
