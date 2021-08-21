//! [557. 反转字符串中的单词 III](https://leetcode-cn.com/problems/reverse-words-in-a-string-iii/)

impl Solution {
    pub fn reverse_words(mut s: String) -> String {
        let bytes = unsafe { s.as_bytes_mut() };
        let mut left = 0;
        let mut right = 0;
        while right < bytes.len() {
            let l = bytes[left] as char;
            if l == ' ' {
                left += 1;
                right = left;
            } else if bytes[right] as char == ' ' {
                bytes[left..right].reverse();
                left = right + 1;
                right = left;
            } else if right == bytes.len() - 1 {
                bytes[left..=right].reverse();
            }
            right += 1;
        }
        s
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::reverse_words("Let's take LeetCode contest".to_owned()),
        "s'teL ekat edoCteeL tsetnoc"
    );
}
