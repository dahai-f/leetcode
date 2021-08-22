//! [5850. 找出数组的最大公约数](https://leetcode-cn.com/problems/find-greatest-common-divisor-of-array/)

impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        for x in nums {
            if x > max {
                max = x;
            }
            if x < min {
                min = x;
            }
        }
        Self::gcd(max, min)
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b > 0 {
            Self::gcd(b, a % b)
        } else {
            a
        }
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::find_gcd(vec![2, 5, 6, 9, 10]), 2);
}
