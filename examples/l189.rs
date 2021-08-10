/// [189. 旋转数组](https://leetcode-cn.com/problems/rotate-array/)

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        let k = k as usize % n;
        // an = bk
        let gcd = Self::gcd(n, k);
        for i in 0..gcd {
            let mut cur = (i + k) % n;
            while cur != i {
                let next = (cur + k) % n;
                nums.swap(i, cur);
                cur = next;
            }
        }
    }

    fn gcd(mut a: usize, mut b: usize) -> usize {
        if a < b {
            std::mem::swap(&mut a, &mut b);
        }
        while b != 0 {
            let r = a % b;
            a = b;
            b = r;
        }
        a
    }
}

struct Solution;

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7];
    Solution::rotate(&mut v, 3);
    assert_eq!(v, vec![5, 6, 7, 1, 2, 3, 4]);
    let mut v = vec![-1, -100, 3, 99];
    Solution::rotate(&mut v, 2);
    assert_eq!(v, vec![3, 99, -1, -100]);
}
