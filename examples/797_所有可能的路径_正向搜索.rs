//! [797. 所有可能的路径](https://leetcode-cn.com/problems/all-paths-from-source-to-target/)

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut temp = vec![];
        Self::search(&graph, &mut temp, 0, &mut result);
        result
    }

    fn search(graph: &[Vec<i32>], temp: &mut Vec<i32>, cur: i32, result: &mut Vec<Vec<i32>>) {
        temp.push(cur);
        if cur == graph.len() as i32 - 1 {
            result.push(temp.clone());
        } else {
            let next = &graph[cur as usize];
            for &next in next {
                Self::search(graph, temp, next, result);
            }
        }
        temp.pop();
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
            vec![0, 3, 4],
            vec![0, 1, 3, 4],
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 4],
        ]
    );
    assert_eq!(
        Solution::all_paths_source_target(vec![vec![1], vec![]]),
        vec![vec![0, 1]]
    );
    assert_eq!(
        Solution::all_paths_source_target(vec![vec![1, 2, 3], vec![2], vec![3], vec![]]),
        vec![vec![0, 1, 2, 3], vec![0, 2, 3], vec![0, 3],]
    );
    assert_eq!(
        Solution::all_paths_source_target(vec![vec![1, 3], vec![2], vec![3], vec![]]),
        vec![vec![0, 1, 2, 3], vec![0, 3],]
    );
}
