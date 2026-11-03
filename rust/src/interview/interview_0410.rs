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
    #[cfg(any())]
    // 法一：暴力枚举
    // 思路：遍历 T1 的每一个节点，判断以该节点为根的子树是否与 T2 完全相同
    pub fn check_sub_tree(t1: Option<Rc<RefCell<TreeNode>>>, t2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // 判断两棵树是否完全相同
        fn is_same(a: &Option<Rc<RefCell<TreeNode>>>, b: &Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (a, b) {
                (None, None) => true,
                (Some(na), Some(nb)) => {
                    let na = na.borrow();
                    let nb = nb.borrow();
                    na.val == nb.val
                        && is_same(&na.left, &nb.left)
                        && is_same(&na.right, &nb.right)
                },
                _ => false
            }
        }

        // 枚举 T1 的每个节点作为起点
        fn dfs(t1: &Option<Rc<RefCell<TreeNode>>>, t2: &Option<Rc<RefCell<TreeNode>>>) -> bool {
            match t1 {
                None => false,
                Some(node) => {
                    if is_same(t1, t2) {
                        return true;
                    }
                    let n = node.borrow();
                    dfs(&n.left, t2) || dfs(&n.right, t2)
                }
            }
        }

        dfs(&t1, &t2)
    }

    // 法二：序列化 + 子串匹配
    // 思路：把两棵树按照前序（或者后序）遍历序列化，然后判断 T2 的序列化是否为 T1 
    // 序列化的子串
    // 注意：无论是前序、中序还是后序遍历，子树节点序列化以后都是连续的，但中序遍历
    // 得到的序列无法还原出树结构。也即不同的树结构可能有相同的中序遍历序列。所以只
    // 能选择前序或者后续遍历的方式序列化
    pub fn check_sub_tree(t1: Option<Rc<RefCell<TreeNode>>>, t2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        
        // 前序遍历序列化
        fn serialize(node: &Option<Rc<RefCell<TreeNode>>>, s: &mut String) {
            match node {
                None => s.push_str("(#)"),
                Some(n) => {
                    let n = n.borrow();
                    s.push('(');
                    s.push_str(&n.val.to_string());
                    s.push(')');

                    serialize(&n.left, s);
                    serialize(&n.right, s);
                }
            }
        }

        let mut s1 = String::new();
        let mut s2 = String::new();
        serialize(&t1, &mut s1);
        serialize(&t2, &mut s2);
        s1.contains(s2.as_str())
    }
}

