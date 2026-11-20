struct Solution;

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

impl Solution {

    fn convert(
        root: Option<Rc<RefCell<TreeNode>>>
    ) -> (
        Option<Rc<RefCell<TreeNode>>>,
        Option<Rc<RefCell<TreeNode>>>
    ) {
        let Some(node) = root else {
            return (None, None);
        };

        // 先取出左右子树，以避免后续借用冲突
        let (left, right) = {
            let mut n = node.borrow_mut();
            (n.left.take(), n.right.take())
        };

        let (lh, lt) = Self::convert(left);
        let (rh, rt) = Self::convert(right);

        // 当前节点左指针悬空，右指针解右子树转换后的头
        {
            let mut n = node.borrow_mut();
            n.left = None;
            n.right = rh;
        }

        // 左子树转换后的尾节点接到当前节点
        if let Some(tail) = lt.as_ref() {
            tail.borrow_mut().right = Some(node.clone());
        }

        let head = lh.or_else(|| Some(node.clone()));
        let tail = rt.or_else(|| Some(node.clone()));

        (head, tail)
    }

    pub fn convert_bi_node(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::convert(root).0
    }
}