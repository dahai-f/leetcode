//! [68. 文本左右对齐](https://leetcode-cn.com/problems/text-justification/)

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let mut result = vec![];
        let mut len = 0;
        let mut words_in_line: Vec<String> = vec![];
        for word in words {
            let new_len = if len == 0 {
                word.len()
            } else {
                len + 1 + word.len()
            };
            if new_len <= max_width {
                len = new_len;
            } else {
                let gap_count = words_in_line.len() - 1;
                let total_space_count = max_width - len + gap_count;
                let (average_space_count, mut rem_space_count) = if gap_count == 0 {
                    (total_space_count, 0)
                } else {
                    (total_space_count / gap_count, total_space_count % gap_count)
                };
                let word_count = words_in_line.len();
                let mut new_line = String::with_capacity(max_width);
                for (i, word) in words_in_line.drain(..).enumerate() {
                    new_line.push_str(word.as_str());
                    if i + 1 < word_count {
                        new_line.extend(
                            (0..(average_space_count
                                + if rem_space_count > 0 {
                                    rem_space_count -= 1;
                                    1
                                } else {
                                    0
                                }))
                                .map(|_i| ' '),
                        );
                    }
                }
                if word_count == 1 {
                    new_line.extend((0..average_space_count).map(|_i| ' '));
                }
                result.push(new_line);

                len = word.len();
            }
            words_in_line.push(word);
        }
        assert!(len > 0);
        let mut last_line = String::with_capacity(max_width);
        let last_line_word_count = words_in_line.len();
        let tail_space_count = max_width - len;
        for (i, word) in words_in_line.into_iter().enumerate() {
            last_line.push_str(word.as_str());
            if i + 1 == last_line_word_count {
                last_line.extend((0..tail_space_count).map(|_i| ' '));
            } else {
                last_line.push(' ');
            }
        }
        result.push(last_line);
        result
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::full_justify(
            vec![
                "This",
                "is",
                "an",
                "example",
                "of",
                "text",
                "justification."
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
            16
        ),
        vec!["This    is    an", "example  of text", "justification.  "]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    );
    assert_eq!(
        Solution::full_justify(
            vec!["What", "must", "be", "acknowledgment", "shall", "be"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            16
        ),
        vec!["What   must   be", "acknowledgment  ", "shall be        "]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    );
    assert_eq!(
        Solution::full_justify(
            vec![
                "Science",
                "is",
                "what",
                "we",
                "understand",
                "well",
                "enough",
                "to",
                "explain",
                "to",
                "a",
                "computer.",
                "Art",
                "is",
                "everything",
                "else",
                "we",
                "do"
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
            20
        ),
        vec![
            "Science  is  what we",
            "understand      well",
            "enough to explain to",
            "a  computer.  Art is",
            "everything  else  we",
            "do                  "
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
    );
}
