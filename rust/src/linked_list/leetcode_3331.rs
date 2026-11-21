struct Solution;

impl Solution {
    pub fn find_subtree_sizes(parent: Vec<i32>, s: String) -> Vec<i32> {
        let n = parent.len();
        let bytes = s.as_bytes();

        // 1. 构建原始树的孩子列表
        let mut orig_children = vec![Vec::new(); n];
        for i in 1..n {
            let p = parent[i] as usize;
            orig_children[p].push(i);
        }

        // 2. 第一次 DFS：在原始树上求出每个节点在新树中的父节点 new_parent
        //    latest[c] 表示当前根到当前节点路径上，字符 c 最近出现的节点编号
        //    用 n 表示“不存在”
        let mut latest = vec![n; 26];
        let mut old_latest = vec![n; n]; // 用于回溯恢复 latest
        let mut new_parent = vec![n; n]; // 根 0 的新父节点保持为 n（表示无父）

        let mut stack = vec![(0usize, false)]; // (节点, 是否为退出事件)
        while let Some((u, is_exit)) = stack.pop() {
            if !is_exit {
                // 进入节点 u
                let cu = (bytes[u] - b'a') as usize;
                old_latest[u] = latest[cu];
                latest[cu] = u;

                // 此时 latest 中包含 u 以及 u 的祖先
                // 对 u 的每个孩子 v，确定 v 在新树中的父节点
                for &v in &orig_children[u] {
                    let cv = (bytes[v] - b'a') as usize;
                    let ancestor = latest[cv];
                    if ancestor != n {
                        // 存在最近的同字符祖先
                        new_parent[v] = ancestor;
                    } else {
                        // 不存在，则父节点保持不变，仍为 u
                        new_parent[v] = u;
                    }
                }

                // 压入退出事件，再压入孩子进入事件（逆序）
                stack.push((u, true));
                for &v in orig_children[u].iter().rev() {
                    stack.push((v, false));
                }
            } else {
                // 退出节点 u，恢复 latest
                let cu = (bytes[u] - b'a') as usize;
                latest[cu] = old_latest[u];
            }
        }

        // 3. 根据 new_parent 构建修改后的新树
        let mut new_children = vec![Vec::new(); n];
        for v in 1..n {
            let p = new_parent[v];
            // 非根节点的新父节点一定存在，且 p < n
            new_children[p].push(v);
        }

        // 4. 第二次 DFS：在新树上后序统计子树大小
        let mut size = vec![1i32; n]; // 每个节点自身贡献 1
        let mut stack2 = vec![(0usize, false)]; // (节点, 是否为退出事件)
        while let Some((u, is_exit)) = stack2.pop() {
            if !is_exit {
                stack2.push((u, true));
                for &v in &new_children[u] {
                    stack2.push((v, false));
                }
            } else {
                // 后序：此时所有孩子的 size 已经计算完毕
                for &v in &new_children[u] {
                    size[u] += size[v];
                }
            }
        }

        size
    }
}