/// [19. 删除链表的倒数第 N 个结点](https://leetcode-cn.com/problems/remove-nth-node-from-end-of-list/)

impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut diff = 0;
        let mut low = &head;
        let mut fast = &head;
        while let Some(f) = fast {
            if diff < n {
                diff += 1;
            } else {
                low = &low.as_ref().unwrap().next;
            }
            fast = &f.next;
        }
        if diff < n {
            head = None;
        } else {
            let nn = low.as_ref().unwrap().next.as_ref().unwrap().next;
            low.as_mut().unwrap().next = nn;
        }
        head
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
