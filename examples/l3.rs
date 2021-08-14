//! [3. 无重复字符的最长子串](https://leetcode-cn.com/problems/longest-substring-without-repeating-characters/)

use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max = 0;
        let mut sub_chars = HashSet::new();
        let mut left = s.chars();
        for right in s.chars() {
            if !sub_chars.insert(right) {
                for left in &mut left {
                    if left == right {
                        break;
                    }
                    sub_chars.remove(&left);
                }
            }
            if sub_chars.len() > max {
                max = sub_chars.len();
            }
        }
        max as i32
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::length_of_longest_substring("abcabcbb".into()), 3);
    assert_eq!(Solution::length_of_longest_substring("".into()), 0);
    assert_eq!(Solution::length_of_longest_substring("bbbbb".into()), 1);
    assert_eq!(Solution::length_of_longest_substring("pwwkew".into()), 3);
}
