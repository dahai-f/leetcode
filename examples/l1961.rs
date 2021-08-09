/// 1961. 检查字符串是否为数组前缀
/// https://leetcode-cn.com/problems/check-if-string-is-a-prefix-of-array/

impl Solution {
    pub fn is_prefix_string(s: String, words: Vec<String>) -> bool {
        let mut s_chars = s.chars();
        let mut cur_s_char = s_chars.next();
        for word in words {
            if cur_s_char.is_none() {
                return true;
            }
            for word_char in word.chars() {
                if let Some(s_char) = cur_s_char {
                    if s_char != word_char {
                        return false;
                    }
                    cur_s_char = s_chars.next();
                } else {
                    return false;
                }
            }
        }
        cur_s_char.is_none()
    }
}

struct Solution;

fn main() {
    assert!(Solution::is_prefix_string(
        "iloveleetcode".to_owned(),
        vec!["i", "love", "leetcode", "apples"]
            .iter()
            .map(|&s| s.to_owned())
            .collect(),
    ));
    assert!(!Solution::is_prefix_string(
        "iloveleetcode".to_owned(),
        vec!["apples", "i", "love", "leetcode"]
            .iter()
            .map(|&s| s.to_owned())
            .collect(),
    ));
    assert!(!Solution::is_prefix_string(
        "a".to_owned(),
        vec!["aa", "aaaa", "banana"]
            .iter()
            .map(|&s| s.to_owned())
            .collect(),
    ));
    assert!(!Solution::is_prefix_string(
        "aaaaaaa".to_owned(),
        vec!["a", "a", "a", "a", "a", "a"]
            .iter()
            .map(|&s| s.to_owned())
            .collect(),
    ));
    assert!(Solution::is_prefix_string(
        "z".to_owned(),
        vec!["z"].iter().map(|&s| s.to_owned()).collect(),
    ));
}
