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
            right: None
        }
    }
}

struct Solution;

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root.as_ref() {
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();

            if let Some(left_node) = left.as_ref() &&
                let Some(right_node) = right.as_ref() {
                if node.borrow().val == left_node.borrow().val + right_node.borrow().val {
                    return true;
                }
            }

        }

        false
    }
}