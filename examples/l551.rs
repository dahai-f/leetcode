//! [551. 学生出勤记录 I](https://leetcode-cn.com/problems/student-attendance-record-i/)

impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut a_count = 0;
        let mut l_count = 0;
        for &x in s.as_bytes().iter() {
            if x == b'A' {
                a_count += 1;
                l_count = 0;
                if a_count >= 2 {
                    return false;
                }
            } else if x == b'L' {
                l_count += 1;
                if l_count >= 3 {
                    return false;
                }
            } else {
                l_count = 0;
            }
        }
        true
    }
}

struct Solution;

fn main() {
    assert!(Solution::check_record("LALL".to_owned()));
    assert!(Solution::check_record("PPALLP".to_owned()));
    assert!(!Solution::check_record("PPALLL".to_owned()));
}
