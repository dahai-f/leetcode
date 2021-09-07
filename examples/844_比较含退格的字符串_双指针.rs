//! [844. 比较含退格的字符串](https://leetcode-cn.com/problems/backspace-string-compare/)

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut s_idx = s.len().wrapping_sub(1);
        let mut t_idx = t.len().wrapping_sub(1);
        loop {
            let s = Self::pre(s, &mut s_idx);
            let t = Self::pre(t, &mut t_idx);
            match (s, t) {
                (None, None) => {
                    break;
                }
                (Some(s), Some(t)) => {
                    if s != t {
                        return false;
                    }
                }
                _ => {
                    return false;
                }
            }
        }

        true
    }

    fn pre(bytes: &[u8], cur: &mut usize) -> Option<u8> {
        let mut skip = 0;
        while *cur < bytes.len() {
            let byte = bytes[*cur];
            *cur = cur.wrapping_sub(1);
            if byte == b'#' {
                skip += 1;
            } else if skip > 0 {
                skip -= 1;
            } else {
                return Some(byte);
            }
        }
        None
    }
}

struct Solution;

fn main() {
    assert!(Solution::backspace_compare(
        "ab#c".to_string(),
        "ad#c".to_string()
    ))
}
