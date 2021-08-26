//! [881. 救生艇](https://leetcode-cn.com/problems/boats-to-save-people/)

impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort_unstable();
        let mut num = 0;
        let mut left = 0;
        let mut right = people.len().wrapping_sub(1);
        while left <= right && right < people.len() {
            num += 1;
            if left == right {
                break;
            } else if people[left] + people[right] > limit {
                right = right.wrapping_sub(1);
            } else {
                left += 1;
                right = right.wrapping_sub(1);
            }
        }
        num
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::num_rescue_boats(vec![1, 2], 3), 1);
    assert_eq!(Solution::num_rescue_boats(vec![3, 2, 2, 1], 3), 3);
    assert_eq!(Solution::num_rescue_boats(vec![3, 5, 3, 4], 5), 4);
}
