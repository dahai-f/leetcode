//! [1221. 分割平衡字符串](https://leetcode-cn.com/problems/split-a-string-in-balanced-strings/)

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let s = s.as_bytes();
        let mut count = 0;
        let mut r_count = 0;
        for &x in s {
            if x == b'R' {
                r_count += 1;
            } else {
                r_count -= 1;
            }
            if r_count == 0 {
                count += 1;
            }
        }
        count
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::balanced_string_split("RLRRLLRLRL".into()), 4);
    assert_eq!(Solution::balanced_string_split("RLLLLRRRLR".into()), 3);
    assert_eq!(Solution::balanced_string_split("LLLLRRRR".into()), 1);
    assert_eq!(Solution::balanced_string_split("RLRRRLLRLL".into()), 2);
}
