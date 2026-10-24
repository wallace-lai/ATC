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
            let node = match node {
                None => return None,
                Some(n) => n,
            };
            let val = node.borrow().val;
            if val == p || val == q {   // 命中了p或者q，返回自身
                return Some(node.clone());
            }

            let left = lca(node.borrow().left.clone(), p, q);
            let right = lca(node.borrow().right.clone(), p, q);
            match (left, right) {
                (Some(_), Some(_)) => Some(node.clone()),   // p和q分居两侧，当前即为 LCA
                (Some(l), None) => Some(l),     // 只在一侧
                (None, Some(r)) => Some(r),
                (None, None) => None,
            }
        }

        // 题目限定
        // p和q均在给定的二叉树中
        // p != q
        // 所有 node.val 互不相同
        lca(root, p.unwrap().borrow().val, q.unwrap().borrow().val)
    }
}