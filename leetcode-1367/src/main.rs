fn main() {}

pub struct Solution;

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
}

use std::cell::RefCell;
use std::rc::Rc;

fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (root, head) {
        (Some(root), Some(head)) => {
            root.borrow().val == head.val
                && (is_sub_path(head.next.clone(), root.borrow().left.clone())
                    || is_sub_path(head.next, root.borrow().right.clone()))
        }
        (_, None) => true,
        (None, Some(_)) => false,
    }
}

fn is_sub_path_inner(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match root {
        Some(root) => {
            is_sub_path_inner(head.clone(), root.borrow().left.clone())
                || is_sub_path_inner(head.clone(), root.borrow().right.clone())
                || is_sub_path(head, Some(root))
        }
        None => false,
    }
}

impl Solution {
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        is_sub_path_inner(head, root)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test() {
        let mut root_right_left = TreeNode::new(10);
        root_right_left.left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let mut root_right = TreeNode::new(1);
        root_right.left = Some(Rc::new(RefCell::new(root_right_left)));
        root_right.right = Some(Rc::new(RefCell::new(TreeNode::new(1))));

        let mut root = TreeNode::new(1);
        root.right = Some(Rc::new(RefCell::new(root_right)));

        let mut head = ListNode::new(1);
        head.next = Some(Box::new(ListNode::new(10)));

        assert_eq!(
            Solution::is_sub_path(Some(Box::new(head)), Some(Rc::new(RefCell::new(root)))),
            true
        );
    }
}
