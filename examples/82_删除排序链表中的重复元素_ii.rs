//! [82. 删除排序链表中的重复元素 II](https://leetcode-cn.com/problems/remove-duplicates-from-sorted-list-ii/)

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

impl From<ListNode> for Option<Box<ListNode>> {
    fn from(list_node: ListNode) -> Self {
        Some(Box::new(list_node))
    }
}

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_head = None;
        let mut new_last = &mut new_head;
        let mut has_same = false;
        while let Some(mut cur) = head {
            let next = cur.next.take();
            if let Some(next) = next {
                if next.val != cur.val {
                    if !has_same {
                        *new_last = Some(cur);
                        new_last = &mut new_last.as_mut().unwrap().next;
                    }
                    has_same = false;
                } else {
                    has_same = true;
                }
                head = Some(next);
            } else {
                if !has_same {
                    *new_last = Some(cur);
                }
                head = None;
            }
        }
        new_head
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::delete_duplicates(ListNode::from(&[1, 2, 3, 3, 4, 4, 5]).into()),
        ListNode::from(&[1, 2, 5]).into()
    );
    assert_eq!(
        Solution::delete_duplicates(ListNode::from(&[1, 1, 1, 2, 3]).into()),
        ListNode::from(&[2, 3]).into()
    );
}
