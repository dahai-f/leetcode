//! [787. K 站中转内最便宜的航班](https://leetcode-cn.com/problems/cheapest-flights-within-k-stops/)

use std::collections::{HashMap, LinkedList};

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let n = n as usize;
        let src = src as usize;
        let dst = dst as usize;
        let mut next_map = HashMap::new();
        for x in flights {
            if let [from, to, price] = *x.as_slice() {
                next_map
                    .entry(from as usize)
                    .or_insert_with(HashMap::new)
                    .insert(to as usize, price);
            }
        }

        let mut distance = vec![i32::MAX; n];
        let mut distance_back = vec![i32::MAX];
        distance[src] = 0;
        let mut q = LinkedList::new();
        let mut q_back = LinkedList::new();
        q.push_back(src);
        for i in 0..=k {
            distance_back.clear();
            distance_back.extend_from_slice(&distance);
            while let Some(from) = q.pop_front() {
                if let Some(next) = next_map.get(&from) {
                    for (&to, &price) in next {
                        distance_back[to] = distance_back[to].min(distance[from] + price);
                        if i < k {
                            q_back.push_back(to);
                        }
                    }
                }
            }
            std::mem::swap(&mut q_back, &mut q);
            std::mem::swap(&mut distance_back, &mut distance);
        }
        if distance[dst] == i32::MAX {
            -1
        } else {
            distance[dst]
        }
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::find_cheapest_price(
            5,
            vec![
                vec![0, 1, 5],
                vec![1, 2, 5],
                vec![0, 3, 2],
                vec![3, 1, 2],
                vec![1, 4, 1],
                vec![4, 2, 1]
            ],
            0,
            2,
            2,
        ),
        7
    );
    assert_eq!(
        Solution::find_cheapest_price(
            3,
            vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
            0,
            2,
            1
        ),
        200
    );
    assert_eq!(
        Solution::find_cheapest_price(
            3,
            vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
            0,
            2,
            0
        ),
        500
    );
}
