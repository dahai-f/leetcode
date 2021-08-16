//! [526. 优美的排列](https://leetcode-cn.com/problems/beautiful-arrangement/)

impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        let n = n as u32;
        let mask_max = 1usize << n;
        let mut d = vec![0; mask_max];
        d[0] = 1;
        for mask in 1..mask_max {
            let num_index = mask.count_ones();
            for i in 0..n {
                let mask_to_remove = 1 << i;
                if (mask & mask_to_remove) != 0
                    && ((i + 1) % num_index == 0 || num_index % (i + 1) == 0)
                {
                    d[mask] += d[mask ^ mask_to_remove];
                }
            }
        }
        d[mask_max - 1]
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::count_arrangement(3), 3);
    assert_eq!(Solution::count_arrangement(2), 2);
}
