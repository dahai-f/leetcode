//! [443. 压缩字符串](https://leetcode-cn.com/problems/string-compression/)

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let n = chars.len();
        let mut read = 0;
        let mut write = 0;
        let mut alpha_location = 0;
        let mut alpha = chars[read];
        read += 1;
        while read <= n {
            if read == n || chars[read] != alpha {
                chars[write] = alpha;
                write += 1;
                let mut num = read - alpha_location;
                if num > 1 {
                    let num_start_location = write;
                    while num > 0 {
                        chars[write] = ((num % 10) as u8 + b'0') as char;
                        write += 1;
                        num /= 10;
                    }
                    chars[num_start_location..write].reverse();
                }
                if read < n {
                    alpha = chars[read];
                    alpha_location = read;
                }
            }
            read += 1;
        }

        write as i32
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::compress(&mut vec!['a', 'a', 'b', 'b', 'c', 'c', 'c']),
        6
    );
    assert_eq!(Solution::compress(&mut vec!['a']), 1);
    assert_eq!(
        Solution::compress(&mut vec![
            'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b'
        ]),
        4
    );
}
