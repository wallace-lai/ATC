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

use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        
        fn lca(node: Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32) -> Option<Rc<RefCell<TreeNode>>> {
            let mut cur = node;
            while let Some(node) = cur {
                let val = node.borrow().val;
                if p < val && q < val {
                    cur = node.borrow().left.clone();
                } else if p > val && q > val {
                    cur = node.borrow().right.clone();
                } else {
                    return Some(node);
                }
            }
            None
        }

        // 题目限定
        // 所有节点的值均是唯一的；
        // p和q为不同节点且均存在于给定的二叉树中
        lca(root, p.unwrap().borrow().val, q.unwrap().borrow().val)
    }
}