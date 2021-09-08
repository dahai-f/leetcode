//! [438. 找到字符串中所有字母异位词](https://leetcode-cn.com/problems/find-all-anagrams-in-a-string/)

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut result = vec![];
        let mut diff = [0; 26];
        let s = s.as_bytes();
        let p = p.as_bytes();
        let mut not_zero_count = 0;

        fn add_diff(byte: u8, diff: &mut [i32], not_zero_count: &mut usize) {
            let d = &mut diff[(byte - b'a') as usize];
            *d += 1;
            if *d == 0 {
                *not_zero_count -= 1;
            } else if *d == 1 {
                *not_zero_count += 1;
            }
        }
        fn sub_diff(byte: u8, diff: &mut [i32], not_zero_count: &mut usize) {
            let d = &mut diff[(byte - b'a') as usize];
            *d -= 1;
            if *d == 0 {
                *not_zero_count -= 1;
            } else if *d == -1 {
                *not_zero_count += 1;
            }
        }

        for &x in p {
            sub_diff(x, &mut diff, &mut not_zero_count);
        }
        let k = p.len().min(s.len());
        for &x in s.iter().take(k) {
            add_diff(x, &mut diff, &mut not_zero_count);
        }
        if not_zero_count == 0 {
            result.push(0);
        }
        for i in 0..(s.len() - k) {
            add_diff(s[i + k], &mut diff, &mut not_zero_count);
            sub_diff(s[i], &mut diff, &mut not_zero_count);
            if not_zero_count == 0 {
                result.push((i + 1) as i32);
            }
        }
        result
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::find_anagrams("baa".to_string(), "aa".to_string()),
        vec![1]
    );
    assert_eq!(
        Solution::find_anagrams("cbaebabacd".to_string(), "abc".to_string()),
        vec![0, 6]
    );
    assert_eq!(
        Solution::find_anagrams("abab".to_string(), "ab".to_string()),
        vec![0, 1, 2]
    );
}
