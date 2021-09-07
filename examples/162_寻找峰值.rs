//! [162. 寻找峰值](https://leetcode-cn.com/problems/find-peak-element/)

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let mid = left + (right - left) / 2;
            let mid_num = nums[mid];
            let mid_right_num = *nums.get(mid.saturating_add(1)).unwrap_or(&i32::MIN);
            if mid_num > mid_right_num {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left as i32
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::find_peak_element(vec![1, 2, 3, 1]), 2);
    assert_eq!(Solution::find_peak_element(vec![1, 2, 1, 3, 5, 6, 4]), 5);
}
