//! [295. 数据流的中位数](https://leetcode-cn.com/problems/find-median-from-data-stream/)

use std::collections::BTreeMap;

struct MedianFinder {
    nums: BTreeMap<i32, usize>,
    n: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    /** initialize your data structure here. */
    fn new() -> Self {
        Self {
            nums: Default::default(),
            n: 0,
        }
    }

    fn add_num(&mut self, num: i32) {
        *self.nums.entry(num).or_default() += 1;
        self.n += 1;
    }

    fn find_median(&self) -> f64 {
        let mut iter = self
            .nums
            .iter()
            .map(|(&num, &count)| (0..count).map(move |_i| num))
            .flatten();
        if self.n & 1 == 1 {
            iter.nth(self.n / 2).unwrap() as f64
        } else {
            let mut iter = iter.skip(self.n / 2 - 1);
            (iter.next().unwrap() + iter.next().unwrap()) as f64 / 2.0
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */

fn main() {
    let mut finder = MedianFinder::new();
    finder.add_num(1);
    finder.add_num(2);
    assert_eq!(
        finder.find_median().partial_cmp(&1.5f64).unwrap(),
        std::cmp::Ordering::Equal
    );
    finder.add_num(3);
    assert_eq!(
        finder.find_median().partial_cmp(&2f64).unwrap(),
        std::cmp::Ordering::Equal
    );
}
