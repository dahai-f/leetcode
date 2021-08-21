//! [206. 反转链表](https://leetcode-cn.com/problems/reverse-linked-list/)

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut head) = head {
            if let Some(mut next) = head.next.take() {
                let n = next.as_mut() as *mut ListNode;
                let new_head = Self::reverse_list(Some(next));
                unsafe {
                    (*n).next = Some(head);
                }
                new_head
            } else {
                Some(head)
            }
        } else {
            None
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
        Solution::reverse_list(ListNode::from(&[1, 2, 3, 4, 5]).into_option()),
        ListNode::from(&[5, 4, 3, 2, 1]).into_option()
    )
}
