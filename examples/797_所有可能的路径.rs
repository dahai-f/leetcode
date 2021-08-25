//! [797. 所有可能的路径](https://leetcode-cn.com/problems/all-paths-from-source-to-target/)

use std::collections::HashMap;

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut to_pre: HashMap<i32, Vec<i32>> = HashMap::new();
        for (i, next) in graph.iter().enumerate() {
            for &next in next.iter() {
                to_pre.entry(next).or_default().push(i as i32);
            }
        }
        let mut result = vec![];
        let mut temp = vec![graph.len() as i32 - 1];
        Self::find_pre(&to_pre, &mut temp, graph.len() as i32 - 1, &mut result);
        result
    }

    fn find_pre(
        to_pre: &HashMap<i32, Vec<i32>>,
        temp: &mut Vec<i32>,
        cur: i32,
        result: &mut Vec<Vec<i32>>,
    ) {
        if cur == 0 {
            let mut temp = temp.clone();
            temp.reverse();
            result.push(temp);
        }
        if let Some(pre) = to_pre.get(&cur) {
            for &pre in pre {
                temp.push(pre);
                Self::find_pre(to_pre, temp, pre, result);
                temp.pop();
            }
        }
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::all_paths_source_target(vec![vec![1, 2], vec![3], vec![3], vec![]]),
        vec![vec![0, 1, 3], vec![0, 2, 3]]
    );
    assert_eq!(
        Solution::all_paths_source_target(vec![
            vec![4, 3, 1],
            vec![3, 2, 4],
            vec![3],
            vec![4],
            vec![]
        ]),
        vec![
            vec![0, 4],
            vec![0, 1, 4],
            vec![0, 3, 4],
            vec![0, 1, 3, 4],
            vec![0, 1, 2, 3, 4]
        ]
    );
    assert_eq!(
        Solution::all_paths_source_target(vec![vec![1], vec![]]),
        vec![vec![0, 1]]
    );
    assert_eq!(
        Solution::all_paths_source_target(vec![vec![1, 2, 3], vec![2], vec![3], vec![]]),
        vec![vec![0, 3], vec![0, 2, 3], vec![0, 1, 2, 3],]
    );
    assert_eq!(
        Solution::all_paths_source_target(vec![vec![1, 3], vec![2], vec![3], vec![]]),
        vec![vec![0, 3], vec![0, 1, 2, 3],]
    );
}
