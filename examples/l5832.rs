//! [5832. 构造元素不等于两相邻元素平均值的数组](https://leetcode-cn.com/contest/weekly-contest-254/problems/array-with-elements-not-equal-to-average-of-neighbors/)
use std::cmp::Ordering;

impl Solution {
    pub fn rearrange_array(mut nums: Vec<i32>) -> Vec<i32> {
        let mid = nums.len() / 2;
        Self::partition_at(&mut nums, mid);
        let mut res = Vec::with_capacity(nums.len());
        let mut left = 0;
        let mut right = mid;
        while right < nums.len() {
            res.push(nums[right]);
            right += 1;
            if left < mid {
                res.push(nums[left]);
                left += 1;
            }
        }
        res
    }

    fn partition_at(vec: &mut [i32], index: usize) {
        let mut left = 0;
        let mut right = vec.len().wrapping_sub(1);
        while left < right {
            while left < right && vec[left] <= vec[right] {
                right = right.wrapping_sub(1);
            }
            if left < right {
                vec.swap(left, right);
            }
            while left < right && vec[left] <= vec[right] {
                left += 1;
            }
            if left < right {
                vec.swap(left, right);
            }
        }

        match left.cmp(&index) {
            Ordering::Less => Self::partition_at(&mut vec[(left + 1)..], index - left - 1),
            Ordering::Equal => {}
            Ordering::Greater => Self::partition_at(&mut vec[..left], index),
        }
    }
}

struct Solution;

fn is_correct(vec: &[i32]) -> bool {
    let mut i = 1;
    let last = vec.len() - 1;
    while i < last {
        if vec[i - 1] - vec[i] == vec[i] - vec[i + 1] {
            println!("{:?}", vec);
            return false;
        }
        i += 1
    }
    true
}

fn main() {
    assert!(is_correct(&Solution::rearrange_array(vec![1, 3, 5, 7])));
    assert!(is_correct(&Solution::rearrange_array(vec![1, 2, 3])));
    assert!(is_correct(&Solution::rearrange_array(vec![1, 2, 3, 4, 5])));
    assert!(is_correct(&Solution::rearrange_array(vec![6, 2, 0, 9, 7])));
}
