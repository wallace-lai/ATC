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
use std::collections::HashMap;

impl Solution {
    // 法一：前缀和 + 回溯 + 哈希表
    // 思路：在当前DFS路径上，设从根到当前节点的前缀和为 curr，那么
    // 如果路径上存在某个祖先节点，其前缀和为 curr - target，则从
    // 该祖先节点的下一个节点到当前节点这段路径的和正好是 target
    // 做法：维护一个哈希表，记录当前 DFS 路径上每种前缀和出现了多少次，
    // 一路向下递归，回溯时撤销即可
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        
        // 从 node 开始 DFS 遍历，返回以 node 为根节点的子树中的路径数
        fn dfs(
            node: &Option<Rc<RefCell<TreeNode>>>,
            curr: i64,
            t: i64,
            presum: &mut HashMap<i64, i32>
        ) -> i32 {
            let node = match node {
                None => return 0,
                Some(n) => n,
            };

            let n = node.borrow();
            let curr_sum = curr + n.val as i64;

            // 统计以当前节点为终点，和为 t 的路径条数
            let mut count = *presum.get(&(curr_sum - t)).unwrap_or(&0);

            // 把当前前缀和加入哈希表
            *presum.entry(curr_sum).or_insert(0) += 1;

            // 递归左右子树
            count += dfs(&n.left, curr_sum, t, presum);
            count += dfs(&n.right, curr_sum, t, presum);

            // 回溯：撤销当前节点对前缀和的贡献
            *presum.entry(curr_sum).or_insert(0) -= 1;

            count
        }
        
        let mut presum: HashMap<i64, i32> = HashMap::new();
        // 空前缀和，方便处理根节点到当前节点整段和为 target 的情况
        presum.insert(0, 1);
        dfs(&root, 0, sum as i64, &mut presum)
    }
}