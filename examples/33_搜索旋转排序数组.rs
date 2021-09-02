//! [33. 搜索旋转排序数组](https://leetcode-cn.com/problems/search-in-rotated-sorted-array/)

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut rot_point = 0;
        while rot_point < nums.len() {
            if rot_point == nums.len() - 1 || nums[rot_point] > nums[rot_point + 1] {
                rot_point += 1;
                break;
            }
            rot_point += 1;
        }
        nums[..rot_point]
            .binary_search(&target)
            .unwrap_or_else(|e| {
                if e == 0 {
                    nums[rot_point..]
                        .binary_search(&target)
                        .unwrap_or(usize::MAX)
                        .saturating_add(rot_point)
                } else {
                    usize::MAX
                }
            }) as i32
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::search(vec![1, 3], 3), 1);
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
}
