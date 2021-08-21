//! [567. 字符串的排列](https://leetcode-cn.com/problems/permutation-in-string/)

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let mut cnt = [0; 26];
        for &x in s1 {
            cnt[(x - b'a') as usize] -= 1;
        }
        let mut left = s2.iter().enumerate();
        let mut left_i = 0;
        for (right_i, &right) in s2.iter().enumerate() {
            let right = (right - b'a') as usize;
            cnt[right] += 1;
            while cnt[right] > 0 {
                let (li, left) = left.next().unwrap();
                left_i = li + 1;
                let left = (left - b'a') as usize;
                cnt[left] -= 1;
            }
            if right_i.wrapping_sub(left_i).wrapping_add(1) == s1.len() {
                return true;
            }
        }
        false
    }
}

struct Solution;

fn main() {
    assert!(!Solution::check_inclusion("ab".into(), "eidboaoo".into()));
    assert!(Solution::check_inclusion("a".into(), "ab".into()));
    assert!(Solution::check_inclusion("adc".into(), "dcda".into()));
    assert!(Solution::check_inclusion("ab".into(), "eidbaooo".into()));
}
