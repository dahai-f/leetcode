//! [977. 有序数组的平方](https://leetcode-cn.com/problems/squares-of-a-sorted-array/)

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::with_capacity(nums.len());
        let mut right = match nums.binary_search(&0) {
            Ok(i) => i,
            Err(i) => i,
        };
        let mut left = right.checked_sub(1);
        loop {
            match (left.map(|l| (l, nums[l])), nums.get(right)) {
                (Some((li, l)), Some(&r)) => {
                    let l_squared = l * l;
                    let r_squared = r * r;
                    if l_squared < r_squared {
                        res.push(l_squared);
                        left = li.checked_sub(1);
                    } else {
                        res.push(r_squared);
                        right += 1;
                    }
                }
                (None, Some(&r)) => {
                    res.push(r * r);
                    right += 1;
                }
                (None, None) => {
                    break;
                }
                (Some((li, l)), None) => {
                    res.push(l * l);
                    left = li.checked_sub(1);
                }
            }
        }
        res
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
        vec![0, 1, 9, 16, 100]
    );
    assert_eq!(
        Solution::sorted_squares(vec![-7, -3, 2, 3, 11]),
        vec![4, 9, 9, 49, 121]
    );
}
