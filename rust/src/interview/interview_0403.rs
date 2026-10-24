
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

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn list_of_depth(tree: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<Box<ListNode>>> {
        
        fn from_slice(slice: &[i32]) -> Option<Box<ListNode>> {
            let mut dummy = Box::new(ListNode { val: -1, next: None });
            let mut p = &mut dummy;

            for val in slice {
                p.next = Some(Box::new(ListNode { val: *val, next: None }));
                p = p.next.as_mut().unwrap();
            }

            dummy.next
        }

        fn level_order(root: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<Vec<i32>>) {
            if root.is_none() { return; }   // 判空
            let mut q = VecDeque::new();
            
            q.push_back(root.unwrap());
            while !q.is_empty() {
                let len = q.len();
                let mut level = Vec::with_capacity(len);

                for _ in 0..len {
                    // 取出当前层的一个节点
                    let curr_rc = q.pop_front().unwrap();
                    // 借用一个不可变引用计数
                    let curr = curr_rc.borrow();
                    // 节点值入当前层的value向量
                    level.push(curr.val);

                    if let Some(left) = curr.left.clone() {
                        q.push_back(left);
                    }
                    if let Some(right) = curr.right.clone() {
                        q.push_back(right);
                    }
                }

                v.push(level);
            }
        }

        let mut ans = Vec::new();
        let mut levels = Vec::new();
        
        level_order(tree, &mut levels);
        for level in levels.iter() {
            ans.push(from_slice(level));
        }

        ans
    }
}