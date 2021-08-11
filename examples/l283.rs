/// [283. 移动零](https://leetcode-cn.com/problems/move-zeroes/)

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = 0;
        let mut j = 0;
        while j < nums.len() {
            if nums[i] == 0 {
                if nums[j] != 0 {
                    nums.swap(i, j);
                    i += 1;
                }
            } else {
                i += 1;
            }
            j += 1;
        }
    }
}

struct Solution;

fn main() {
    let mut v = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut v);
    assert_eq!(v, vec![1, 3, 12, 0, 0]);
}
