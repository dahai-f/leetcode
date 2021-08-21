//! [617. 合并二叉树](https://leetcode-cn.com/problems/merge-two-binary-trees/)

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

impl Solution {
    pub fn merge_trees(
        node1: Option<Rc<RefCell<TreeNode>>>,
        node2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (node1, node2) {
            (None, None) => None,
            (node1 @ Some(_), None) => node1,
            (None, node2 @ Some(_)) => node2,
            (Some(node1), Some(node2)) => {
                let mut node1_ref = node1.deref().borrow_mut();
                let mut node2_ref = node2.deref().borrow_mut();
                node1_ref.val += node2_ref.val;
                node1_ref.left = Self::merge_trees(node1_ref.left.take(), node2_ref.left.take());
                node1_ref.right = Self::merge_trees(node1_ref.right.take(), node2_ref.right.take());
                drop(node1_ref);
                Some(node1)
            }
        }
    }
}

struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn into_rc(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }

    pub fn into_op(self) -> Option<Rc<RefCell<Self>>> {
        Some(self.into_rc())
    }
}

impl From<Vec<i32>> for TreeNode {
    fn from(vec: Vec<i32>) -> Self {
        fn create_node(vec: &[i32], index: usize) -> TreeNode {
            let mut node = TreeNode::new(vec[index]);
            let left_child = index * 2 + 1;
            node.left = if left_child < vec.len() {
                let val = vec[left_child];
                if val < 0 {
                    None
                } else {
                    Some(Rc::new(RefCell::new(create_node(vec, left_child))))
                }
            } else {
                None
            };
            let right_child = left_child + 1;
            node.right = if right_child < vec.len() {
                let val = vec[right_child];
                if val < 0 {
                    None
                } else {
                    Some(Rc::new(RefCell::new(create_node(vec, right_child))))
                }
            } else {
                None
            };
            node
        }
        create_node(&vec, 0)
    }
}

fn main() {
    assert_eq!(
        Solution::merge_trees(
            TreeNode::from(vec![1, 3, 2, 5]).into_op(),
            TreeNode::from(vec![2, 1, 3, -1, 4, -1, 7]).into_op(),
        ),
        (TreeNode::from(vec![3, 4, 5, 5, 4, -1, 7]).into_op())
    );
}
