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
use std::collections::HashSet;

impl Solution {
    pub fn inorder(root: &Option<Rc<RefCell<TreeNode>>>, k: i32, set: &mut HashSet<i32>, ans: &mut bool) {
        if *ans == true {
            return;
        }

        if let Some(node) = root.as_ref() {
            let left = node.borrow().left.clone();
            Self::inorder(&left, k, set, ans);

            let val = node.borrow().val;
            if set.contains(&(k - val)) {
                *ans = true;
                return;
            }
            set.insert(val);

            let right = node.borrow().right.clone();
            Self::inorder(&right, k, set, ans);
        }
    }

    // 8ms，击败100%
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut ans = false;
        let mut set: HashSet<i32> = HashSet::new();
        
        Self::inorder(&root, k, &mut set, &mut ans);
        ans
    }
}