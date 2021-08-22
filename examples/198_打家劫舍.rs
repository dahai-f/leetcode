//! [198. 打家劫舍](https://leetcode-cn.com/problems/house-robber/)

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return nums.iter().copied().max().unwrap();
        }

        let mut nums = nums.into_iter();
        let mut pre_not_rob = nums.next().unwrap();
        let mut pre_rob = nums.next().unwrap();

        for num in nums {
            let rob = pre_not_rob + num;
            let not_rob = pre_not_rob.max(pre_rob);
            pre_not_rob = not_rob;
            pre_rob = rob;
        }

        pre_not_rob.max(pre_rob)
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
}
