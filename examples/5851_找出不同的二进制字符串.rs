//! [5851. 找出不同的二进制字符串](https://leetcode-cn.com/problems/find-unique-binary-string/solution/)

use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let n = nums.len();
        let nums: HashSet<u32> = nums
            .into_iter()
            .map(|s| {
                let s = s.as_bytes();
                let mut num = 0;
                for (i, &c) in s.iter().enumerate() {
                    if c == b'1' {
                        num |= 1 << (n - 1 - i)
                    }
                }
                num
            })
            .collect();

        for x in 0..(1 << n) {
            if !nums.contains(&x) {
                let mut result = vec![];
                for i in (0..n).rev() {
                    if (1 << i) & x != 0 {
                        result.push('1');
                    } else {
                        result.push('0');
                    }
                }
                return String::from_iter(result.iter());
            }
        }

        "".to_owned()
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::find_different_binary_string(
            vec!["01", "10"].iter().map(|&s| s.to_owned()).collect()
        ),
        "00".to_owned()
    );
    assert_eq!(
        Solution::find_different_binary_string(
            vec!["00", "01"].iter().map(|&s| s.to_owned()).collect()
        ),
        "10".to_owned()
    );
}
