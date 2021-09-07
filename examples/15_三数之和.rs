//! [15. 三数之和](https://leetcode-cn.com/problems/3sum/)

use std::cmp::Ordering;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let pre_index = |mut cur: usize| {
            let cur_num = nums[cur];
            cur = cur.wrapping_sub(1);
            while cur < nums.len() && nums[cur] == cur_num {
                cur = cur.wrapping_sub(1);
            }
            cur
        };
        let next_index = |mut cur: usize| {
            let cur_num = nums[cur];
            cur = cur.saturating_add(1);
            while cur < nums.len() && nums[cur] == cur_num {
                cur = cur.saturating_add(1);
            }
            cur
        };

        let mut res = vec![];
        let last_i = nums.len().saturating_sub(2);
        let mut i = 0;
        while i < last_i {
            let i_num = nums[i];
            if i > 0 && nums[i - 1] == i_num {
                continue;
            }

            let mut left = i + 1;
            let mut right = nums.len() - 1;
            while left < nums.len() && right < nums.len() && left < right {
                let mid = left + (right - left) / 2;
                if mid == left {
                    if (nums[i] + nums[left] + nums[right]) == 0 {
                        res.push(vec![nums[i], nums[left], nums[right]]);
                    }
                    break;
                }
                match (nums[i] + nums[left] + nums[mid]).cmp(&0) {
                    Ordering::Less => match (nums[i] + nums[mid] + nums[right]).cmp(&0) {
                        Ordering::Less => {
                            left = next_index(mid);
                        }
                        Ordering::Equal => {
                            res.push(vec![nums[i], nums[mid], nums[right]]);
                            left = next_index(mid);
                            right = pre_index(right);
                        }
                        Ordering::Greater => match (nums[i] + nums[left] + nums[right]).cmp(&0) {
                            Ordering::Less => {
                                left = next_index(left);
                            }
                            Ordering::Equal => {
                                res.push(vec![nums[i], nums[left], nums[right]]);
                                left = next_index(left);
                                right = pre_index(right);
                            }
                            Ordering::Greater => {
                                right = pre_index(right);
                            }
                        },
                    },
                    Ordering::Equal => {
                        res.push(vec![nums[i], nums[left], nums[mid]]);
                        left = next_index(left);
                        right = pre_index(mid);
                    }
                    Ordering::Greater => {
                        right = pre_index(mid);
                    }
                }
            }
            i = next_index(i);
        }
        res
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::three_sum(vec![1, 2, -2, -1]),
        Vec::<Vec<i32>>::new()
    );
    assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
    assert_eq!(
        Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
        vec![vec![-1, -1, 2], vec![-1, 0, 1]]
    );
    assert_eq!(Solution::three_sum(vec![]), Vec::<Vec<i32>>::new());
    assert_eq!(Solution::three_sum(vec![0]), Vec::<Vec<i32>>::new());
}
