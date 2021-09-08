//! [209. 长度最小的子数组](https://leetcode-cn.com/problems/minimum-size-subarray-sum/)

impl Solution {
    pub fn min_sub_array_len(target: i32, mut nums: Vec<i32>) -> i32 {
        let mut min_len = usize::MAX;
        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }
        for i in 0..nums.len() {
            let pre = nums.get(i.wrapping_sub(1)).copied().unwrap_or(0);
            let target = pre + target;
            let len = nums[i..].binary_search(&target).unwrap_or_else(|e| e);
            if len + i < nums.len() {
                let len = len + 1;
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
    assert_eq!(
        Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]),
        0
    );
    assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
    assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
}
