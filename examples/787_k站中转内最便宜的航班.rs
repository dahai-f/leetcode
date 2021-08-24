//! [787. K 站中转内最便宜的航班](https://leetcode-cn.com/problems/cheapest-flights-within-k-stops/)

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let n = n as usize;
        let src = src as usize;
        let dst = dst as usize;
        let mut distance = vec![i32::MAX; n];
        distance[src] = 0;
        let mut distance_back = distance.clone();
        for _i in 0..=k {
            for x in flights.iter() {
                let from = x[0] as usize;
                let to = x[1] as usize;
                let price = x[2];
                distance_back[to] = distance_back[to].min(distance[from].saturating_add(price));
            }
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
            3,
            vec![vec![0, 1, 2], vec![1, 2, 1], vec![2, 0, 10]],
            1,
            2,
            1,
        ),
        1
    );
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
