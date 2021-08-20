//! [541. 反转字符串 II](https://leetcode-cn.com/problems/reverse-string-ii/)

impl Solution {
    pub fn reverse_str(mut s: String, k: i32) -> String {
        let k = k as usize;
        let bytes = unsafe { s.as_bytes_mut() };
        let n = bytes.len();
        let mut i = 0;
        while i < n {
            bytes[i..(i + k).min(n)].reverse();
            i += 2 * k;
        }
        s
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::reverse_str("abcdefg".to_owned(), 2), "bacdfeg");
    assert_eq!(Solution::reverse_str("abcd".to_owned(), 2), "bacd");
}
