//! [713. 乘积小于K的子数组](https://leetcode-cn.com/problems/subarray-product-less-than-k/)

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;

        for i in 0..nums.len() {
            let mut mul = 1;
            for &x in nums.iter().skip(i) {
                mul *= x;
                if mul < k {
                    count += 1;
                } else {
                    break;
                }
            }
        }

        count
    }
}

struct Solution;

fn main() {
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
