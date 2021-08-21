//! [876. 链表的中间结点](https://leetcode-cn.com/problems/middle-of-the-linked-list/)

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut a = &head;
        let mut b = &head;
        while let Some(bb) = b {
            b = &bb.next;
            if let Some(bb) = b {
                b = &bb.next;
                a = &a.as_ref().unwrap().next;
            }
        }
        a.clone()
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

    fn get(&self, index: usize) -> Option<Box<ListNode>> {
        let mut cur = self;
        for _i in 0..index {
            cur = cur.next.as_ref().unwrap();
        }
        Some(Box::new(cur.clone()))
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
    let list = Box::<ListNode>::from(&[1, 2, 3, 4, 5]);
    let ans = list.get(2);
    assert_eq!(Solution::middle_node(Some(list)), ans);
    let list = Box::<ListNode>::from(&[1, 2, 3, 4, 5, 6]);
    let ans = list.get(3);
    assert_eq!(Solution::middle_node(Some(list)), ans);
}
