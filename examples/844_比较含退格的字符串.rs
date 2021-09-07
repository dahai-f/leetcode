//! [844. 比较含退格的字符串](https://leetcode-cn.com/problems/backspace-string-compare/)
impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut s_stack = vec![];
        let mut t_stack = vec![];
        for &x in s.as_bytes() {
            if x == b'#' {
                s_stack.pop();
            } else {
                s_stack.push(x);
            }
        }
        for &x in t.as_bytes() {
            if x == b'#' {
                t_stack.pop();
            } else {
                t_stack.push(x);
            }
        }
        s_stack == t_stack
    }
}
struct Solution;

fn main() {
    assert!(Solution::backspace_compare(
        "ab#c".to_string(),
        "ad#c".to_string()
    ))
}
