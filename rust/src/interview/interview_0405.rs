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
    // 法一：递归中序
    // BST的中序遍历序列必须严格递增，用 pre 记录前驱节点的值，每访问一个节点就检查 pre < cur
    // pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        
    //     fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, pre: &mut Option<i32>) -> bool {
    //         let node = match node {
    //             None => return true,
    //             Some(n) => n,
    //         };
    //         let node = node.borrow();

    //         // 遍历左子树
    //         if !dfs(&node.left, pre) { return false; }

    //         // 访问当前节点：必须严格大于前驱
    //         if let Some(p) = *pre {
    //             if p >= node.val { return false; }
    //         }
    //         *pre = Some(node.val);

    //         // 遍历右子树
    //         dfs(&node.right, pre)
    //     }

    //     let mut pre = None;
    //     dfs(&root, &mut pre)
    // }

    // 法二：迭代中序
    // pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    //     let mut stk = Vec::new();
    //     let mut cur = root;
    //     let mut pre = None;

    //     while cur.is_some() || !stk.is_empty() {
    //         // 一路向左，把左链全部压栈
    //         while let Some(node) = cur {
    //             cur = node.borrow().left.clone();
    //             stk.push(node);
    //         }

    //         // 弹出栈顶，即当前最左节点
    //         let node = stk.pop().unwrap();
    //         let val = node.borrow().val;

    //         // 检查严格递增
    //         if let Some(p) = pre {
    //             if p >= val { return false; }
    //         }
    //         pre = Some(val);

    //         // 转向右子树
    //         cur = node.borrow().right.clone();
    //     }

    //     true
    // }

    // 法三：自顶向下DFS
    // 把每个节点必须满足的约束表示为一个开区间，沿递归路径往下转递
    // pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        
    //     // 合法区间为 lower < val < upper
    //     fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, lower: i64, upper: i64) -> bool {
    //         match node {
    //             None => true,
    //             Some(n) => {
    //                 let val = n.borrow().val as i64;
    //                 if val <= lower || val >= upper { return false; }
    //                 dfs(n.borrow().left.as_ref(), lower, val) &&
    //                 dfs(n.borrow().right.as_ref(), val, upper)
    //             }
    //         }
    //     }

    //     dfs(root.as_ref(), i64::MIN, i64::MAX)
    // }

    // 法四：自底向上DFS（性能比法三差）
    // 递归函数返回子树的信息，而不是向下传递约束
    // 递归函数返回 Result<(i64, i64), ()>，语义如下：
    // 1. Ok((min_val, max_val)) - 子树合法，且树中最小值为min_val，最大值为max_val
    // 2. Ok((i64::MAX, i64::MIN)) - 空子树
    // 3. Err(()) - 子树非法
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> Result<(i64, i64), ()> {
            let node = match node {
                None => return Ok((i64::MAX, i64::MIN)),    // 空子树，值域为空集
                Some(n) => n.borrow(),
            };
            let val = node.val as i64;

            let (left_min, left_max) = dfs(&node.left)?;
            let (right_min, right_max) = dfs(&node.right)?;
            if left_max >= val || right_min <= val {
                return Err(());
            }

            Ok((
                left_min.min(val),  // 最小值为：左子树最小值和当前值取最小
                right_max.max(val), // 最大值为：右子树最大值和当前值取最大
            ))
        }

        dfs(&root).is_ok()
    }
}