//! [1109. 航班预订统计](https://leetcode-cn.com/problems/corporate-flight-bookings/)

impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut diff = vec![0; n];
        for booking in bookings {
            let first = (booking[0] - 1) as usize;
            let last = booking[1] as usize;
            let num = booking[2];
            diff[first] += num;
            if last < diff.len() {
                diff[last] -= num;
            }
        }

        for i in 1..n {
            diff[i] += diff[i - 1];
        }
        diff
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::corp_flight_bookings(vec![vec![1, 2, 10], vec![2, 3, 20], vec![2, 5, 25]], 5),
        vec![10, 55, 45, 25, 25]
    );
    assert_eq!(
        Solution::corp_flight_bookings(vec![vec![1, 2, 10], vec![2, 2, 15]], 2),
        vec![10, 25]
    );
}
