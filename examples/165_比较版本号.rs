//! [165. 比较版本号](https://leetcode-cn.com/problems/compare-version-numbers/)

use std::cmp::Ordering;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut version1 = version1
            .split('.')
            .map(|segment| segment.parse::<i32>().unwrap());
        let mut version2 = version2
            .split('.')
            .map(|segment| segment.parse::<i32>().unwrap());
        loop {
            let (ver1, ver2) = match (version1.next(), version2.next()) {
                (Some(ver1), Some(ver2)) => (ver1, ver2),
                (None, Some(ver2)) => (0, ver2),
                (Some(ver1), None) => (ver1, 0),
                (None, None) => {
                    break;
                }
            };
            match ver1.cmp(&ver2) {
                Ordering::Less => {
                    return -1;
                }
                Ordering::Equal => {}
                Ordering::Greater => {
                    return 1;
                }
            }
        }
        0
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::compare_version("1.01".to_owned(), "1.001".to_owned()),
        0
    );
    assert_eq!(
        Solution::compare_version("1.0".to_owned(), "1.0.0".to_owned()),
        0
    );
    assert_eq!(
        Solution::compare_version("0.1".to_owned(), "1.1".to_owned()),
        -1
    );
    assert_eq!(
        Solution::compare_version("1.0.1".to_owned(), "1".to_owned()),
        1
    );
    assert_eq!(
        Solution::compare_version("7.5.2.4".to_owned(), "7.5.3".to_owned()),
        -1
    );
}
