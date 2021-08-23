//! [34. 在排序数组中查找元素的第一个和最后一个位置](https://leetcode-cn.com/problems/find-first-and-last-position-of-element-in-sorted-array/)

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let left = Self::partition_point(&nums, |num| num < target);
        if left >= nums.len() || nums[left] != target {
            vec![-1, -1]
        } else {
            let right = Self::partition_point(&nums, |num| num <= target);
            vec![left as i32, right as i32 - 1]
        }
    }

    fn partition_point(nums: &[i32], f: impl Fn(i32) -> bool) -> usize {
        let mut left = 0;
        let mut right = nums.len();
        while left < right {
            let mid = left + (right - left) / 2;
            if f(nums[mid]) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
        vec![3, 4]
    );
}
