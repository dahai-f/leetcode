//! [11. 盛最多水的容器](https://leetcode-cn.com/problems/container-with-most-water/)

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut left = 0;
        let mut right = height.len().wrapping_sub(1);
        while left < right {
            let left_height = height[left];
            let right_height = height[right];
            let area = left_height.min(right_height) * (right - left) as i32;
            if area > max_area {
                max_area = area;
            }
            if left_height < right_height {
                left += 1;
            } else {
                right = right.wrapping_sub(1);
            }
        }
        max_area
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
}
