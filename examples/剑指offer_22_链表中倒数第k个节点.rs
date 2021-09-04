//! [剑指 Offer 22. 链表中倒数第k个节点](https://leetcode-cn.com/problems/lian-biao-zhong-dao-shu-di-kge-jie-dian-lcof/)

use std::cell::UnsafeCell;

impl Solution {
    pub fn get_kth_from_end(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let head = UnsafeCell::new(head);
        let mut left = unsafe { &mut *head.get() };
        let mut right: &_ = unsafe { &*head.get() };
        for _i in 0..k {
            if let Some(r) = right {
                right = &r.next;
            } else {
                return left.take();
            }
        }
        while let Some(r) = right {
            right = &r.next;
            left = &mut left.as_mut().unwrap().next;
        }
        left.take()
    }
}

struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn into_option(self) -> Option<Box<Self>> {
        Some(Box::new(self))
    }
}

impl<const S: usize> From<&[i32; S]> for ListNode {
    fn from(slice: &[i32; S]) -> Self {
        let mut pre = None;
        for x in slice.iter().rev().copied() {
            let mut new_head = ListNode::new(x);
            new_head.next = pre.map(Box::new);
            pre = Some(new_head);
        }
        pre.unwrap()
    }
}

fn main() {
    assert_eq!(
        Solution::get_kth_from_end(ListNode::from(&[1, 2, 3, 4, 5]).into_option(), 2),
        ListNode::from(&[4, 5]).into_option()
    );
}
