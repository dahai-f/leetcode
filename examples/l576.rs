//! [576. 出界的路径数](https://leetcode-cn.com/problems/out-of-boundary-paths/)

use std::collections::HashMap;

impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        let (m, n, max_move, start_row, start_column) = (
            m as usize,
            n as usize,
            max_move as usize,
            start_row as usize,
            start_column as usize,
        );
        let mut sum = 0;

        let mut move_to = |pre_num: i32,
                           cur_move: &mut HashMap<(usize, usize), i32>,
                           new_row: usize,
                           new_column: usize| {
            let sum = if new_row >= m || new_column >= n {
                &mut sum
            } else {
                cur_move.entry((new_row, new_column)).or_default()
            };
            const MOD: i32 = 1000000007;
            *sum = (*sum + pre_num) % MOD;
        };

        let mut pre = HashMap::new();
        let mut cur = HashMap::new();
        pre.insert((start_row, start_column), 1);
        for _i in 0..max_move {
            for ((row, column), num) in pre.drain() {
                move_to(num, &mut cur, row, column.wrapping_sub(1));
                move_to(num, &mut cur, row, column + 1);
                move_to(num, &mut cur, row.wrapping_sub(1), column);
                move_to(num, &mut cur, row + 1, column);
            }
            std::mem::swap(&mut pre, &mut cur);
        }
        sum
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::find_paths(2, 2, 2, 0, 0), 6);
    assert_eq!(Solution::find_paths(1, 3, 3, 0, 1), 12);
}
