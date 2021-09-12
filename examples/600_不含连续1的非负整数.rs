//! [600. 不含连续1的非负整数](https://leetcode-cn.com/problems/non-negative-integers-without-consecutive-ones/)

impl Solution {
    pub fn find_integers(n: i32) -> i32 {
        let l = 32 - n.leading_zeros() as usize;
        let mut dp = vec![0; (l + 1).max(2)];
        dp[0] = 1;
        dp[1] = 1;
        for i in 2..=l {
            dp[i] = dp[i - 1] + dp[i - 2];
        }

        let mut result = 0;
        let mut pre_is_one = false;
        for i in (0..l).rev() {
            if ((1 << i) & n) != 0 {
                result += dp[i + 1];
                if pre_is_one {
                    break;
                }
                pre_is_one = true;
            } else {
                pre_is_one = false;
            }

            if i == 0 {
                result += 1;
            }
        }

        result
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::find_integers(4), 4);
    assert_eq!(Solution::find_integers(6), 5);
    assert_eq!(Solution::find_integers(5), 5);
}
