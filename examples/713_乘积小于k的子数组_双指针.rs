//! [713. 乘积小于K的子数组](https://leetcode-cn.com/problems/subarray-product-less-than-k/)

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut left = 0;
        let mut right = 0;
        let mut mul = 1;
        while right < nums.len() {
            mul *= nums[right];
            right += 1;
            while mul >= k && left < right {
                mul /= nums[left];
                left += 1;
            }
            count += right - left;
        }
        count as i32
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::num_subarray_product_less_than_k(
            vec![10, 3, 3, 7, 2, 9, 7, 4, 7, 2, 8, 6, 5, 1, 5],
            30
        ),
        26
    );
    assert_eq!(
        Solution::num_subarray_product_less_than_k(
            vec![10, 9, 10, 4, 3, 8, 3, 3, 6, 2, 10, 10, 9, 3],
            19
        ),
        18
    );
    assert_eq!(
        Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100),
        8
    );
    assert_eq!(
        Solution::num_subarray_product_less_than_k(vec![1, 2, 3], 0),
        0
    );
}
