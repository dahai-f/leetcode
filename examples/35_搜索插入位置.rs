//! [35. 搜索插入位置](https://leetcode-cn.com/problems/search-insert-position/)

use std::cmp::Ordering;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();
        while left < right {
            let mid = left + (right - left) / 2;
            match nums[mid].cmp(&target) {
                Ordering::Less => {
                    left = mid + 1;
                }
                Ordering::Equal => {
                    return mid as i32;
                }
                Ordering::Greater => {
                    right = mid;
                }
            }
        }
        left as i32
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
    assert_eq!(Solution::search_insert(vec![1], 0), 0);
}
