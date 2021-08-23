//! [1646. 获取生成数组中的最大值](https://leetcode-cn.com/problems/get-maximum-in-generated-array/)

impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        let n = n as usize;
        let mut result = Vec::with_capacity(n + 1);
        result.push(0);
        result.push(1);
        for i in 2..=n {
            result.push(if i % 2 == 1 {
                result[i / 2] + result[i / 2 + 1]
            } else {
                result[i / 2]
            });
        }
        result.iter().copied().max().unwrap()
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::get_maximum_generated(15), 5);
    assert_eq!(Solution::get_maximum_generated(0), 0);
    assert_eq!(Solution::get_maximum_generated(7), 3);
    assert_eq!(Solution::get_maximum_generated(2), 1);
    assert_eq!(Solution::get_maximum_generated(3), 2);
}
