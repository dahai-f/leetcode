//! [567. 字符串的排列](https://leetcode-cn.com/problems/permutation-in-string/)

use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut is_in_s1 = HashSet::new();
        let mut not_in_window = HashMap::new();
        let mut s1_chars_count = 0;
        for c in s1.chars() {
            is_in_s1.insert(c);
            *not_in_window.entry(c).or_insert(0) += 1;
            s1_chars_count += 1;
        }
        let mut left = s2.chars();
        let mut right = s2.chars();
        for _i in 0..s1_chars_count {
            if let Some(right) = right.next() {
                Self::enter_window(&mut not_in_window, &is_in_s1, right);
            }
        }

        loop {
            if not_in_window.is_empty() {
                return true;
            }
            match right.next() {
                None => {
                    return false;
                }
                Some(right) => {
                    Self::enter_window(&mut not_in_window, &is_in_s1, right);
                }
            }
            let left = left.next().unwrap();
            Self::exit_window(&mut not_in_window, &is_in_s1, left);
        }
    }

    fn enter_window(not_in_window: &mut HashMap<char, i32>, is_in_s1: &HashSet<char>, right: char) {
        if is_in_s1.contains(&right) {
            match not_in_window.entry(right) {
                Occupied(mut occ) => {
                    let count = occ.get_mut();
                    *count -= 1;
                    if *count == 0 {
                        occ.remove();
                    }
                }
                Vacant(vac) => {
                    vac.insert(-1);
                }
            }
        }
    }

    fn exit_window(not_in_window: &mut HashMap<char, i32>, is_in_s1: &HashSet<char>, left: char) {
        if is_in_s1.contains(&left) {
            match not_in_window.entry(left) {
                Occupied(mut occ) => {
                    let count = occ.get_mut();
                    if *count == -1 {
                        occ.remove();
                    } else {
                        *count += 1;
                    }
                }
                Vacant(vac) => {
                    vac.insert(1);
                }
            };
        }
    }
}

struct Solution;

fn main() {
    assert!(Solution::check_inclusion("adc".into(), "dcda".into()));
    assert!(Solution::check_inclusion("ab".into(), "eidbaooo".into()));
    assert!(!Solution::check_inclusion("ab".into(), "eidboaoo".into()));
}
