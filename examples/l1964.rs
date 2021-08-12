#![feature(map_first_last)]

/// [1964. 找出到每个位置为止最长的有效障碍赛跑路线](https://leetcode-cn.com/problems/remove-stones-to-minimize-the-total/)
use std::collections::BTreeMap;

impl Solution {
    pub fn longest_obstacle_course_at_each_position(obstacles: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::with_capacity(obstacles.len());
        let mut max_of_obstacle = BTreeMap::new();
        for i in 0..obstacles.len() {
            let cur = obstacles[i];
            let mut b = max_of_obstacle.split_off(&cur);
            let length = match b.pop_first() {
                None => match max_of_obstacle.last_key_value() {
                    None => 0,
                    Some((_obstacle, &max_length)) => max_length,
                },
                Some((obstacle, max_length)) => {
                    if obstacle != cur {
                        match max_of_obstacle.last_key_value() {
                            None => 0,
                            Some((_obstacle, &max_length)) => max_length,
                        }
                    } else {
                        max_length
                    }
                }
            };
            let length = length + 1;
            b.insert(cur, length);
            ans.push(length);
            max_of_obstacle.append(&mut b);
        }
        ans
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::longest_obstacle_course_at_each_position(vec![1, 2, 3, 2]),
        vec![1, 2, 3, 3]
    );
    assert_eq!(
        Solution::longest_obstacle_course_at_each_position(vec![2, 2, 1]),
        vec![1, 2, 1]
    );
    assert_eq!(
        Solution::longest_obstacle_course_at_each_position(vec![3, 1, 5, 6, 4, 2]),
        vec![1, 1, 2, 3, 2, 2]
    );
    assert_eq!(
        Solution::longest_obstacle_course_at_each_position(vec![5, 1, 5, 5, 1, 3, 4, 5, 1, 4]),
        vec![1, 1, 2, 3, 2, 3, 4, 5, 3, 5]
    );
}
