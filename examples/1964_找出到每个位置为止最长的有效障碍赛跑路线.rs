//! [1964. 找出到每个位置为止最长的有效障碍赛跑路线](https://leetcode-cn.com/problems/find-the-longest-valid-obstacle-course-at-each-position/)
use std::cmp::Ordering::{Greater, Less};

impl Solution {
    pub fn longest_obstacle_course_at_each_position(mut obstacles: Vec<i32>) -> Vec<i32> {
        let mut d = Vec::with_capacity(obstacles.len());
        for cur in obstacles.iter_mut() {
            let point = d
                .binary_search_by(|x| if *x <= *cur { Less } else { Greater })
                .unwrap_or_else(|i| i);
            if point >= d.len() {
                d.push(*cur);
            } else {
                d[point] = *cur;
            }
            *cur = (point + 1) as i32;
        }
        obstacles
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
