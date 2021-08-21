//! [784. 字母大小写全排列](https://leetcode-cn.com/problems/letter-case-permutation/)

impl Solution {
    pub fn letter_case_permutation(mut s: String) -> Vec<String> {
        let bytes = unsafe { s.as_bytes_mut() };
        let mut result = vec![];
        Self::dfs(bytes, 0, &mut result);
        result
    }

    fn dfs(bytes: &mut [u8], i: usize, result: &mut Vec<String>) {
        if i == bytes.len() {
            result.push(unsafe { String::from_utf8_unchecked(bytes.to_vec()) });
            return;
        }
        let byte = bytes[i];
        if byte.is_ascii_alphabetic() {
            bytes[i] = byte.to_ascii_lowercase();
            Self::dfs(bytes, i + 1, result);
            bytes[i] = byte.to_ascii_uppercase();
        }
        Self::dfs(bytes, i + 1, result);
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::letter_case_permutation("a1b2".to_owned()),
        ["a1b2", "a1B2", "A1b2", "A1B2"]
    );
    assert_eq!(
        Solution::letter_case_permutation("3z4".to_owned()),
        ["3z4", "3Z4"]
    );
    assert_eq!(
        Solution::letter_case_permutation("12345".to_owned()),
        ["12345"]
    );
}
