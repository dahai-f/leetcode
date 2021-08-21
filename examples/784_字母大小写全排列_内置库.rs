//! [784. 字母大小写全排列](https://leetcode-cn.com/problems/letter-case-permutation/)

use std::iter::Product;

struct StringVec(Vec<String>);

impl From<StringVec> for Vec<String> {
    fn from(v: StringVec) -> Self {
        v.0
    }
}

impl Product<Vec<u8>> for StringVec {
    fn product<I: Iterator<Item = Vec<u8>>>(mut iter: I) -> Self {
        let mut result: Vec<String> = vec![];
        if let Some(first) = iter.next() {
            for x in first {
                result.push(String::from(x as char));
            }
        }
        for x in iter {
            let x: Vec<u8> = x;
            let x_n = x.len();
            let n = result.len();
            for i in 0..x_n {
                let x = x[i];
                for j in (n * i)..(n * (i + 1)) {
                    if i < x_n - 1 {
                        result.push(result[j].clone());
                    }
                    result[j].push(x as char);
                }
            }
        }
        Self(result)
    }
}

impl Solution {
    pub fn letter_case_permutation(mut s: String) -> Vec<String> {
        let bytes = unsafe { s.as_bytes_mut() };
        bytes
            .iter()
            .map(|&byte| {
                if byte.is_ascii_alphabetic() {
                    vec![byte.to_ascii_lowercase(), byte.to_ascii_uppercase()]
                } else {
                    vec![byte]
                }
            })
            .product::<StringVec>()
            .into()
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
