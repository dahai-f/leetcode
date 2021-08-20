//! [19. 删除链表的倒数第 N 个结点](https://leetcode-cn.com/problems/remove-nth-node-from-end-of-list/)
use std::cell::UnsafeCell;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;
        let dummy = UnsafeCell::new(Some(dummy));
        let mut low = unsafe { &mut *dummy.get() };
        let mut fast = &unsafe { &*dummy.get() }.as_ref().unwrap().next;
        for _i in 0..n {
            fast = &fast.as_ref().unwrap().next;
        }

        while let Some(f) = fast.as_ref() {
            fast = &f.next;
            low = &mut low.as_mut().unwrap().next;
        }

        let low = low.as_mut().unwrap();
        let low_next = low.next.take();
        low.next = low_next.unwrap().next;
        dummy.into_inner().unwrap().next
    }
}

struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

//noinspection DuplicatedCode
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl<const S: usize> From<&[i32; S]> for Box<ListNode> {
    fn from(data: &[i32; S]) -> Self {
        let mut next = None;
        for &x in data.iter().rev() {
            let mut pre = Box::new(ListNode::new(x));
            pre.next = next;
            next = Some(pre);
        }
        next.unwrap()
    }
}

fn main() {
    assert_eq!(
        Solution::remove_nth_from_end(Some(Box::<ListNode>::from(&[1, 2])), 2),
        Some(Box::<ListNode>::from(&[2]))
    );
    assert_eq!(
        Solution::remove_nth_from_end(Some(Box::<ListNode>::from(&[1, 2, 3, 4, 5])), 2),
        Some(Box::<ListNode>::from(&[1, 2, 3, 5]))
    );
    assert_eq!(
        Solution::remove_nth_from_end(Some(Box::<ListNode>::from(&[1])), 1),
        None
    );
    assert_eq!(
        Solution::remove_nth_from_end(Some(Box::<ListNode>::from(&[1, 2])), 1),
        Some(Box::<ListNode>::from(&[1]))
    );
}
