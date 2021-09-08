//! [986. 区间列表的交集](https://leetcode-cn.com/problems/interval-list-intersections/)

impl Solution {
    pub fn interval_intersection(
        first_list: Vec<Vec<i32>>,
        second_list: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let mut first_idx = 0;
        let mut second_idx = 0;
        let mut result = vec![];
        while first_idx < first_list.len() && second_idx < second_list.len() {
            let first = &first_list[first_idx];
            let second = &second_list[second_idx];
            let left = first[0].max(second[0]);
            let right = first[1].min(second[1]);
            if left <= right {
                result.push(vec![left, right]);
            }
            if right == first[1] {
                first_idx += 1;
            } else {
                second_idx += 1;
            }
        }
        result
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::interval_intersection(
            vec![vec![0, 2], vec![5, 10], vec![13, 23], vec![24, 25]],
            vec![vec![1, 5], vec![8, 12], vec![15, 24], vec![25, 26]]
        ),
        vec![
            vec![1, 2],
            vec![5, 5],
            vec![8, 10],
            vec![15, 23],
            vec![24, 24],
            vec![25, 25]
        ]
    );
}
