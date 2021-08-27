//! [295. 数据流的中位数](https://leetcode-cn.com/problems/find-median-from-data-stream/)

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct MedianFinder {
    left: BinaryHeap<i32>,
    right: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    /** initialize your data structure here. */
    fn new() -> Self {
        Self {
            left: Default::default(),
            right: Default::default(),
        }
    }

    fn add_num(&mut self, num: i32) {
        let add_to_right = {
            if let Some(&left_max) = self.left.peek() {
                num > left_max
            } else {
                true
            }
        };
        if add_to_right {
            self.right.push(Reverse(num));
            if self.right.len() > self.left.len() {
                self.left.push(self.right.pop().unwrap().0);
            }
        } else {
            self.left.push(num);
            if self.left.len() > self.right.len() + 1 {
                self.right.push(Reverse(self.left.pop().unwrap()));
            }
        }
    }

    fn find_median(&self) -> f64 {
        if self.left.len() > self.right.len() {
            *self.left.peek().unwrap() as f64
        } else {
            (self.left.peek().unwrap_or(&0) + self.right.peek().unwrap_or(&Reverse(0i32)).0) as f64
                / 2.0
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
