//! [77. 组合](https://leetcode-cn.com/problems/combinations/)

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut temp = vec![];
        let mut res = vec![];
        Self::dfs(n, k, &mut temp, &mut res);
        res
    }

    fn dfs(n: i32, k: i32, temp: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if n < k || k < 1 {
            res.push(temp.clone());
            return;
        }

        temp.push(n);
        Self::dfs(n - 1, k - 1, temp, res);
        temp.pop();
        if n > k {
            Self::dfs(n - 1, k, temp, res);
        }
    }
}

struct Solution;

fn is_correct(vec: Vec<Vec<i32>>, n: usize, k: usize) -> bool {
    let num = (0..k).map(|i| n - i).product::<usize>() / (1..=k).product::<usize>();
    if vec.len() != num {
        println!("{:?}", vec);
        false
    } else {
        true
    }
}

fn main() {
    assert!(is_correct(Solution::combine(4, 2), 4, 2));
    assert!(is_correct(Solution::combine(1, 1), 1, 1));
}
