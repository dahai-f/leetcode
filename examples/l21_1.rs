//! [21. 合并两个有序链表](https://leetcode-cn.com/problems/merge-two-sorted-lists/)

impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (l1 @ Some(_), None) => l1,
            (None, l2 @ Some(_)) => l2,
            (Some(mut inner_1), Some(mut inner_2)) => {
                if inner_1.val < inner_2.val {
                    inner_1.next = Self::merge_two_lists(inner_1.next, Some(inner_2));
                    Some(inner_1)
                } else {
                    inner_2.next = Self::merge_two_lists(Some(inner_1), inner_2.next);
                    Some(inner_2)
                }
            }
        }
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
        Solution::merge_two_lists(
            ListNode::from(&[1, 2, 4]).into_option(),
            ListNode::from(&[1, 3, 4]).into_option(),
        ),
        ListNode::from(&[1, 1, 2, 3, 4, 4]).into_option()
    )
}
