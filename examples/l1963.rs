/// [1963. 使字符串平衡的最小交换次数](https://leetcode-cn.com/problems/minimum-number-of-swaps-to-make-the-string-balanced/)

impl Solution {
    pub fn min_swaps(s: String) -> i32 {}
}

struct Solution;

fn main() {
    assert_eq!(Solution::min_swaps("][][".to_owned()), 1);
    assert_eq!(Solution::min_swaps("]]][[[".to_owned()), 2);
    assert_eq!(Solution::min_swaps("[]".to_owned()), 0);
    assert_eq!(Solution::min_swaps("".to_owned()), 0);
}
