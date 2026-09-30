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
    // 返回值：子树的结点数和结点值总和
    pub fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> (i32, i32) {
        if let Some(node) = root.as_ref() {
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            let (lnum, lsum) = Self::dfs(&left, ans);
            let (rnum, rsum) = Self::dfs(&right, ans);
            let val = node.borrow().val;

            let num = lnum + rnum + 1;
            let sum = lsum + rsum + val;
            if val == sum / num {
                *ans += 1;
            }

            return (num, sum);
        }

        (0, 0)
    }

    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        Self::dfs(&root, &mut ans);
        ans
    }
}