//! [295. 数据流的中位数](https://leetcode-cn.com/problems/find-median-from-data-stream/)

struct MedianFinder {
    nums: Vec<i32>,
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
        }
    }

    fn add_num(&mut self, num: i32) {
        let i = self.nums.binary_search(&num).unwrap_or_else(|e| e);
        self.nums.insert(i, num);
    }

    fn find_median(&self) -> f64 {
        let n = self.nums.len();
        if n & 1 == 1 {
            self.nums[n / 2] as f64
        } else {
            (self.nums[n / 2 - 1] + self.nums[n / 2]) as f64 / 2.0
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
