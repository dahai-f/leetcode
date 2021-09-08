//! [438. 找到字符串中所有字母异位词](https://leetcode-cn.com/problems/find-all-anagrams-in-a-string/)

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut result = vec![];
        let mut diff = [0; 26];
        let s = s.as_bytes();
        let p = p.as_bytes();
        for &x in p {
            diff[(x - b'a') as usize] -= 1;
        }
        let k = p.len().min(s.len());
        for i in 0..k {
            diff[(s[i] - b'a') as usize] += 1;
        }
        for i in k..=s.len() {
            let sub = p.iter().all(|p| diff[(*p - b'a') as usize] == 0);
            if sub {
                result.push((i - k) as i32);
            }
            if i < s.len() {
                diff[(s[i] - b'a') as usize] += 1;
                diff[(s[i - k] - b'a') as usize] -= 1;
            }
        }
        result
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::find_anagrams("cbaebabacd".to_string(), "abc".to_string()),
        vec![0, 6]
    );
    assert_eq!(
        Solution::find_anagrams("abab".to_string(), "ab".to_string()),
        vec![0, 1, 2]
    );
}
