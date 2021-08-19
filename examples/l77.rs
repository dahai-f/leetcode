//! [77. 组合](https://leetcode-cn.com/problems/combinations/)

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let num = (0..k).map(|i| n - i).product::<i32>() / (1..k).product::<i32>();
        let mut res = Vec::with_capacity(num as usize);
        Self::cases(&mut res, Vec::with_capacity(k as usize), 0, k, n);
        res
    }

    fn cases(res: &mut Vec<Vec<i32>>, mut cur: Vec<i32>, i: i32, k: i32, n: i32) {
        if k == 0 {
            res.push(cur);
            return;
        }

        cur.push(i + 1);
        for x in (i + 1)..(n - k) {
            let new_case = if x < n - k - 1 {
                let mut new_case = Vec::with_capacity(cur.capacity());
                new_case.extend(cur.iter());
                new_case
            } else {
                vec![]
            };
            Self::cases(res, cur, x, k - 1, n);
            cur = new_case;
        }
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::combine(4, 2),
        vec![
            vec![2, 4],
            vec![3, 4],
            vec![2, 3],
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
        ]
    );
    assert_eq!(Solution::combine(1, 1), vec![vec![1]]);
}
