//! [153. 寻找旋转排序数组中的最小值](https://leetcode-cn.com/problems/find-minimum-in-rotated-sorted-array/)
use std::cmp::Ordering;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let mid = left + (right - left) / 2;
            match nums[mid].cmp(&nums[right]) {
                Ordering::Less => {
                    right = mid;
                }
                Ordering::Equal => {
                    unreachable!()
                }
                Ordering::Greater => {
                    left = mid + 1;
                }
            }
        }
        nums[left]
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::find_min(vec![3, 1, 2]), 1);
    assert_eq!(Solution::find_min(vec![1, 2]), 1);
    assert_eq!(Solution::find_min(vec![2, 1]), 1);
    assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
    assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
    assert_eq!(Solution::find_min(vec![11, 13, 15, 17]), 11);
}
