//! [77. 组合](https://leetcode-cn.com/problems/combinations/)

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        if n < k || k < 1 {
            return vec![vec![]];
        }
        let mut with_me = Self::combine(n - 1, k - 1);
        with_me.iter_mut().for_each(|case| case.push(n));
        if n > k {
            let mut with_out_me = Self::combine(n - 1, k);
            with_me.append(&mut with_out_me);
        }
        with_me
    }
}

struct Solution;

fn is_correct(vec: Vec<Vec<i32>>, n: usize, k: usize) -> bool {
    let num = (0..k).map(|i| n - i).product::<usize>() / (1..=k).product::<usize>();
    vec.len() == num
}

fn main() {
    assert!(is_correct(Solution::combine(4, 2), 4, 2));
    assert!(is_correct(Solution::combine(1, 1), 1, 1));
}
