//! [1967. 作为子字符串出现在单词中的字符串数目](https://leetcode-cn.com/problems/number-of-strings-that-appear-as-substrings-in-word/)

impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        patterns
            .iter()
            .filter(|pattern| Self::check_pattern(pattern.as_bytes(), word.as_bytes()))
            .count() as i32
    }

    fn check_pattern(pattern: &[u8], word: &[u8]) -> bool {
        let mut pi = vec![0; pattern.len()];
        let mut j = 0;
        for (i, &c) in pattern.iter().enumerate().skip(1) {
            while j > 0 && pattern[j] != c {
                j = pi[j - 1];
            }
            if pattern[j] == c {
                j += 1;
            }
            pi[i] = j;
        }
        let mut j = 0;
        for &c in word {
            while j > 0 && pattern[j] != c {
                j = pi[j - 1];
            }
            if pattern[j] == c {
                j += 1;
            }
            if j == pattern.len() {
                return true;
            }
        }
        j == pattern.len()
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::num_of_strings(
            vec![
                "pkzkmqi",
                "zz",
                "lpdx",
                "kls",
                "itpkzkmqivpp",
                "alzivm",
                "zk",
                "tpkzkmqiv",
                "soboa",
                "z",
                "mddcfbfjqwxs",
                "cvqfjhllkuvomhtzf",
                "raoffpqqtsmbuvvpaf",
            ]
            .iter()
            .map(|&s| s.to_owned())
            .collect(),
            "dnvitpkzkmqivppozzv".to_owned(),
        ),
        6
    );
    assert_eq!(
        Solution::num_of_strings(
            vec!["a", "abc", "bc", "d"]
                .iter()
                .map(|&s| s.to_owned())
                .collect(),
            "abc".to_owned(),
        ),
        3
    );
    assert_eq!(
        Solution::num_of_strings(
            vec!["a", "b", "c"].iter().map(|&s| s.to_owned()).collect(),
            "aaaaabbbbb".to_owned(),
        ),
        2
    );
    assert_eq!(
        Solution::num_of_strings(
            vec!["a", "a", "a"].iter().map(|&s| s.to_owned()).collect(),
            "ab".to_owned(),
        ),
        3
    );
}
