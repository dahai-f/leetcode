/// [19. 删除链表的倒数第 N 个结点](https://leetcode-cn.com/problems/remove-nth-node-from-end-of-list/)
use std::cell::UnsafeCell;
use std::cmp::Ordering;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut diff = 0;
        let head = UnsafeCell::new(head);
        let mut low = unsafe { &mut *head.get() };
        let mut fast = unsafe { &*head.get() };

        while let Some(f) = fast.as_ref() {
            if diff <= n {
                diff += 1;
            } else {
                low = &mut low.as_mut().unwrap().next;
            }
            fast = &f.next;
        }

        match diff.cmp(&n) {
            Ordering::Less => None,
            Ordering::Equal => low.take().unwrap().next,
            Ordering::Greater => {
                let next = low.as_mut().unwrap().next.take();
                low.as_mut().unwrap().next = next.unwrap().next;
                head.into_inner()
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
