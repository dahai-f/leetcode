//! [5863. 统计特殊四元组](https://leetcode-cn.com/contest/weekly-contest-257/problems/count-special-quadruplets/)

impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        Self::count_rec(&nums, 0, 0, &mut count);
        count
    }

    fn count_rec(nums: &[i32], selected_count: usize, selected_sum: i32, count: &mut i32) {
        if nums.is_empty() {
            return;
        }
        if selected_count == 3 && selected_sum == nums[0] {
            *count += 1;
        }
        Self::count_rec(&nums[1..], selected_count, selected_sum, count);
        if selected_count < 3 {
            Self::count_rec(
                &nums[1..],
                selected_count + 1,
                selected_sum + nums[0],
                count,
            );
        }
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::count_quadruplets(vec![9, 6, 8, 23, 39, 23]), 2);
    assert_eq!(
        Solution::count_quadruplets(vec![28, 8, 49, 85, 37, 90, 20, 8]),
        1
    );
    assert_eq!(Solution::count_quadruplets(vec![1, 2, 3, 6]), 1);
    assert_eq!(Solution::count_quadruplets(vec![3, 3, 6, 4, 5]), 0);
    assert_eq!(Solution::count_quadruplets(vec![1, 1, 1, 3, 5]), 4);
}
