//! [345. 反转字符串中的元音字母](https://leetcode-cn.com/problems/reverse-vowels-of-a-string/)

impl Solution {
    pub fn reverse_vowels(mut s: String) -> String {
        const VOWEL: [u8; 5] = [b'a', b'e', b'i', b'o', b'u'];
        let bytes = unsafe { s.as_bytes_mut() };
        let mut left = 0;
        let mut right = bytes.len().wrapping_sub(1);
        while left < right {
            if !VOWEL.contains(&bytes[left].to_ascii_lowercase()) {
                left += 1;
                continue;
            }
            if !VOWEL.contains(&bytes[right].to_ascii_lowercase()) {
                right = right.wrapping_sub(1);
                continue;
            }
            bytes.swap(left, right);
            left += 1;
            right = right.wrapping_sub(1);
        }
        s
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::reverse_vowels("hello".into()), "holle".to_owned());
    assert_eq!(
        Solution::reverse_vowels("leetcode".into()),
        "leotcede".to_owned()
    );
}
