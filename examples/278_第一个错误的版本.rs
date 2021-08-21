//! [278. 第一个错误的版本](https://leetcode-cn.com/problems/first-bad-version/)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut left = 0;
        let mut right = n;
        while left < right {
            let mid = left + (right - left) / 2;
            if self.isBadVersion(mid + 1) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left + 1
    }
}

struct Solution;

impl Solution {
    fn isBadVersion(&self, version: i32) -> bool {
        version >= 1702766719
    }
}

fn main() {
    let res = Solution.first_bad_version(2126753390);
    println!("{}", res);
}
