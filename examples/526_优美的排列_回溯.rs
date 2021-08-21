//! [526. 优美的排列](https://leetcode-cn.com/problems/beautiful-arrangement/)

impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        let n = n as usize;
        let mut visited = vec![false; n];
        let matched: Vec<Vec<usize>> = (1..=n)
            .map(|i| {
                (0..n)
                    .filter(|j| i % (j + 1) == 0 || (j + 1) % i == 0)
                    .collect()
            })
            .collect();
        let mut num = 0;
        Self::backtrace(&mut visited, &matched, 0, &mut num);
        num
    }

    fn backtrace(visited: &mut Vec<bool>, matched: &[Vec<usize>], index: usize, num: &mut i32) {
        if index == visited.len() {
            *num += 1;
            return;
        }
        for &i in matched[index].iter() {
            if !visited[i] {
                visited[i] = true;
                Self::backtrace(visited, matched, index + 1, num);
                visited[i] = false;
            }
        }
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::count_arrangement(3), 3);
    assert_eq!(Solution::count_arrangement(2), 2);
}
