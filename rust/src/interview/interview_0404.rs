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
    // 法一：自底向上DFS
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        
        // 若子树已经不平衡，使用 ? 短路，父节点不再需要进一步计算
        fn height(node: &Option<Rc<RefCell<TreeNode>>>) -> Result<i32, ()> {
            let node = match node {
                None => return Ok(0),   // 空子树树高为0
                Some(n) => n.borrow(),
            };

            let lh = height(&node.left)?;   // 左子树不平衡，? 短路立刻返Err
            let rh = height(&node.right)?;
            if lh.abs_diff(rh) > 1 {
                return Err(());
            }
            Ok(1 + lh.max(rh))
        }

        height(&root).is_ok()
    }
}