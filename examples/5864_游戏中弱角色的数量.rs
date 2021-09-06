//! [5864. 游戏中弱角色的数量](https://leetcode-cn.com/contest/weekly-contest-257/problems/the-number-of-weak-characters-in-the-game/)
use std::cmp::Ordering;

impl Solution {
    pub fn number_of_weak_characters(mut properties: Vec<Vec<i32>>) -> i32 {
        let cmp = |a: &Vec<i32>, b: &Vec<i32>| match (a[0].cmp(&b[0]), a[1].cmp(&b[1])) {
            (Ordering::Less, _) => Ordering::Greater,
            (Ordering::Greater, _) => Ordering::Less,
            (Ordering::Equal, defense) => defense,
        };
        properties.sort_unstable_by(cmp);

        let mut count = 0;
        let mut max_d = 0;
        for x in properties {
            let cur_d = x[1];
            if cur_d >= max_d {
                max_d = cur_d;
            } else {
                count += 1;
            }
        }

        count
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::number_of_weak_characters(vec![vec![5, 5], vec![6, 3], vec![3, 6]]),
        0
    );
    assert_eq!(
        Solution::number_of_weak_characters(vec![vec![2, 2], vec![3, 3]]),
        1
    );
    assert_eq!(
        Solution::number_of_weak_characters(vec![vec![1, 5], vec![10, 4], vec![4, 3]]),
        1
    );
}
