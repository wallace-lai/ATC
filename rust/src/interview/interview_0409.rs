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
    // 法一：基于候选节点集合的回溯（双端队列）
    pub fn bst_sequences_1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        
        fn dfs_candidates(
            candidates: &mut Vec<Rc<RefCell<TreeNode>>>,
            path: &mut Vec<i32>,
            ans: &mut Vec<Vec<i32>>
        ) {
            // 终止条件：候选集合为空，所有节点已经按照某种合法顺序全部排入了path
            if candidates.is_empty() {
                ans.push(path.clone());
                return;
            }

            let len = candidates.len();
            for i in 0..len {
                // 1. 取出第i个候选节点
                let node = candidates.remove(i);
                let val = node.borrow().val;
                path.push(val);

                // 2. 取出当前候选节点的左右孩子
                let (left, right) = {
                    let b = node.borrow();
                    (b.left.clone(), b.right.clone())
                };

                let mut added = 0;
                if let Some(l) = left {
                    candidates.push(l);
                    added += 1;
                }
                if let Some(r) = right {
                    candidates.push(r);
                    added += 1;
                }

                // 3. 递归
                dfs_candidates(candidates, path, ans);

                // 4. 回溯：先弹出后加入的孩子（数量用 added 记录）
                for _ in 0..added {
                    candidates.pop();
                }
                path.pop();

                // 5. 把 node 放回原来的位置
                // 经过步骤4和5才算把选择第 i 个节点的操作给全部回退了
                candidates.insert(i, node);
            }
        }

        let mut ans = Vec::new();
        match root {
            None => { ans.push(vec![]); },
            Some(r) => {
                let mut candidates = vec![r];
                let mut path = Vec::new();
                dfs_candidates(&mut candidates, &mut path, &mut ans);
            }
        }

        ans
    }

    // 法二：递归交织左右子树
    pub fn bst_sequences(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        
        // 把 left[i..] 和 right[j..] 按各自原顺序交织
        fn interleave(
            left: &[i32], i: usize,
            right: &[i32], j: usize,
            path: &mut Vec<i32>,
            ans: &mut Vec<Vec<i32>>
        ) {
            // 终止条件：一侧耗尽，另一侧只能按照原顺序原样追加
            if i == left.len() || j == right.len() {
                let mut result = path.clone();
                if i < left.len() {
                    result.extend_from_slice(&left[i..]);
                }
                if j < right.len() {
                    result.extend_from_slice(&right[j..]);
                }
                ans.push(result);
                return;
            }

            // 选择：从左边选一个
            path.push(left[i]);
            interleave(left, i + 1, right, j, path, ans);
            path.pop();

            // 选择：从右边选一个
            path.push(right[j]);
            interleave(left, i, right, j + 1, path, ans);
            path.pop();
        }

        fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
            match root {
                None => { return vec![vec![]]; },
                Some(r) => {
                    // 先借出左右孩子和val
                    let (left, right, root_val) = {
                        let b = r.borrow();
                        (b.left.clone(), b.right.clone(), b.val)
                    };

                    let left_seqs = dfs(left);
                    let right_seqs = dfs(right);

                    let mut ans = Vec::new();
                    for l in &left_seqs {
                        for r in &right_seqs {
                            let mut merged = Vec::new();
                            interleave(l, 0, r, 0, &mut Vec::new(), &mut merged);

                            // 根节点必须排在最前面
                            for m in merged {
                                let mut full = Vec::with_capacity(m.len() + 1);
                                full.push(root_val);
                                full.extend_from_slice(&m);
                                ans.push(full);
                            }
                        }
                    }

                    return ans;
                }
            }
        }

        dfs(root)
    }
}