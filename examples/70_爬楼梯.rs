//! [70. 爬楼梯](https://leetcode-cn.com/problems/climbing-stairs/)

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut pre_1 = 1;
        let mut pre_2 = 1;
        for _ in 2..=n {
            let cur = pre_1 + pre_2;
            pre_1 = pre_2;
            pre_2 = cur;
        }
        pre_2
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::climb_stairs(2), 2);
    assert_eq!(Solution::climb_stairs(3), 3);
}
