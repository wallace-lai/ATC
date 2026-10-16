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
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = -1;
        let root_value = root.as_ref().unwrap().borrow().val;

        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, ans: &mut i32, root_value: i32) {
            if let Some(n) = node {
                let value = n.borrow().val;
                let left = n.borrow().left.clone();
                let right = n.borrow().right.clone();

                if *ans != -1 && value >= *ans {
                    return;
                }
                if value > root_value {
                    *ans = value;
                }

                dfs(left, ans, root_value);
                dfs(right, ans, root_value);
            }
        }

        dfs(root, &mut ans, root_value);
        ans
    }
}
