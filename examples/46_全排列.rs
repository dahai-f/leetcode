//! [46. 全排列](https://leetcode-cn.com/problems/permutations/)

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let not_used_mask = 2usize.pow(nums.len() as u32) - 1;
        let mut temp = vec![];
        let mut res = vec![];
        Self::cases(not_used_mask, &nums, &mut temp, &mut res);
        res
    }

    fn cases(not_used_mask: usize, nums: &[i32], temp: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if not_used_mask == 0 {
            res.push(temp.clone());
        }

        for i in 0..nums.len() {
            let i_mask = 1 << i;
            if i_mask & not_used_mask != 0 {
                temp.push(nums[i]);
                Self::cases(not_used_mask ^ i_mask, nums, temp, res);
                temp.pop();
            }
        }
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::permute(vec![1, 2, 3]),
        vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1]
        ]
    );
}
