// Definition for a binary tree node.
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
    // 中序遍历，获得有序数组
    pub fn inorder1(root: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
        if let Some(node) = root.as_ref() {
            let left_child = node.borrow().left.clone();
            Self::inorder1(&left_child, v);

            v.push(node.borrow().val);

            let right_child = node.borrow().right.clone();
            Self::inorder1(&right_child, v);
        }
    }

    // 法一：中序遍历后生成新的树
    // 0ms - 执行时间击败100%
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut v = Vec::<i32>::new();
        Self::inorder1(&root, &mut v);

        // println!("v is {:?}", v);

        let mut curr = None;
        for &val in v.iter().rev() {
            // 创建新结点，其右子节点指向已构建的链
            let node = Rc::new(RefCell::new(TreeNode {
                val,
                left: None,
                right: curr,
            }));

            curr = Some(node);
        }

        curr
    }

    // 法二：在中序遍历的过程中直接修改结点指向
    // pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    //     // ...
    // }
}