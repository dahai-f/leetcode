/// [1964. 找出到每个位置为止最长的有效障碍赛跑路线](https://leetcode-cn.com/problems/find-the-longest-valid-obstacle-course-at-each-position/)

impl Solution {
    pub fn longest_obstacle_course_at_each_position(mut obstacles: Vec<i32>) -> Vec<i32> {
        let mut max_obstacle_pairs: Vec<(i32, i32)> = Vec::with_capacity(obstacles.len());
        for cur in obstacles.iter_mut() {
            let mut length = None;
            for i in (0..max_obstacle_pairs.len()).rev() {
                let (max, obstacle) = max_obstacle_pairs[i];
                if obstacle <= *cur {
                    let new_max = max + 1;
                    let pos = match max_obstacle_pairs
                        .binary_search_by_key(&new_max, |(max, _obstacle)| *max)
                    {
                        Ok(i) => i,
                        Err(i) => i,
                    };
                    max_obstacle_pairs.insert(pos, (new_max, *cur));
                    length = Some(new_max);
                    break;
                }
            }
            let length = match length {
                None => {
                    max_obstacle_pairs.insert(0, (1, *cur));
                    1
                }
                Some(length) => length,
            };
            *cur = length;
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
