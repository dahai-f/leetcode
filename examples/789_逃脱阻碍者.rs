//! [789. 逃脱阻碍者](https://leetcode-cn.com/problems/escape-the-ghosts/)

impl Solution {
    pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let st = target[0].abs() + target[1].abs();
        ghosts.into_iter().all(|g| {
            let gt = (g[0] - target[0]).abs() + (g[1] - target[1]).abs();
            gt > st
        })
    }
}

struct Solution;

fn main() {
    assert!(!Solution::escape_ghosts(vec![vec![2, 2]], vec![1, 1]));
    assert!(Solution::escape_ghosts(
        vec![
            vec![-1, 0],
            vec![0, 1],
            vec![-1, 0],
            vec![0, 1],
            vec![-1, 0]
        ],
        vec![0, 0]
    ));
    assert!(!Solution::escape_ghosts(
        vec![
            vec![5, 0],
            vec![-10, -2],
            vec![0, -5],
            vec![-2, -2],
            vec![-7, 1]
        ],
        vec![7, 7]
    ));
    assert!(Solution::escape_ghosts(
        vec![vec![1, 0], vec![0, 3]],
        vec![0, 1]
    ));
    assert!(!Solution::escape_ghosts(vec![vec![1, 0]], vec![2, 0]));
    assert!(!Solution::escape_ghosts(vec![vec![2, 0]], vec![1, 0]));
}
