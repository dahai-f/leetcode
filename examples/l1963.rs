//! [1963. 使字符串平衡的最小交换次数](https://leetcode-cn.com/problems/minimum-number-of-swaps-to-make-the-string-balanced/)

impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let s = s.as_bytes();
        let mut min_cnt = 0;
        let mut cnt = 0;
        for &c in s.iter() {
            if c == b'[' {
                cnt += 1;
            } else {
                cnt -= 1;
                min_cnt = cnt.min(min_cnt);
            }
        }
        (-min_cnt + 1) / 2
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::min_swaps("][][".to_owned()), 1);
    assert_eq!(Solution::min_swaps("]]][[[".to_owned()), 2);
    assert_eq!(Solution::min_swaps("[]".to_owned()), 0);
    assert_eq!(Solution::min_swaps("".to_owned()), 0);
}
