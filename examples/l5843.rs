//! [5843. 作为子字符串出现在单词中的字符串数目](https://leetcode-cn.com/problems/number-of-strings-that-appear-as-substrings-in-word/)
impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        patterns
            .iter()
            .filter(|&pattern| word.contains(pattern))
            .count() as i32
    }
}

struct Solution;

fn main() {
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
}
