//! [77. 组合](https://leetcode-cn.com/problems/combinations/)

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut temp: Vec<i32> = (1..=k).collect();
        let mut res = vec![];
        let mut j = 0;
        while j < k {
            res.push(temp.clone());

            ++temp[j];
            j -= 1;
        }
        res
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
