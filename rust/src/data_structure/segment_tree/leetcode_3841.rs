struct Fenwick {
    tree: Vec<i32>,
    n: usize,
}

impl Fenwick {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![0; n + 1],
            n,
        }
    }

    // 单点异或
    fn add(&mut self, mut i: usize, val: i32) {
        while i <= self.n {
            self.tree[i] ^= val;
            i += i & i.wrapping_neg();
        }
    }

    // 区间异或 [l, r]（差分写法）
    fn range_xor(&mut self, l: usize, r: usize, val: i32) {
        self.add(l, val);
        if r + 1 <= self.n {
            self.add(r + 1, val);
        }
    }

    // 单点查询（前缀异或和）
    fn point_query(&self, mut i: usize) -> i32 {
        let mut res = 0;
        while i > 0 {
            res ^= self.tree[i];
            i -= i & i.wrapping_neg();
        }
        res
    }
}

struct Solution;

impl Solution {
    pub fn palindrome_path(
        n: i32,
        edges: Vec<Vec<i32>>,
        s: String,
        queries: Vec<String>,
    ) -> Vec<bool> {
        let n = n as usize;
        let s = s.as_bytes();

        // ---------- 1. 建图 ----------
        let mut g = vec![vec![]; n];
        for e in &edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            g[u].push(v);
            g[v].push(u);
        }

        // ---------- 2. 一次迭代 DFS：求 depth / parent / tin / tout / base_mask ----------
        let mut parent = vec![0usize; n];
        let mut depth = vec![0usize; n];
        let mut tin = vec![0usize; n];
        let mut tout = vec![0usize; n];
        let mut base_mask = vec![0i32; n];

        let mut clock = 0usize;
        // 栈元素：(节点, 父节点, 是否退栈)
        let mut stack: Vec<(usize, usize, bool)> = vec![(0, 0, false)];

        base_mask[0] = 1 << (s[0] - b'a');
        parent[0] = 0;
        depth[0] = 0;

        while let Some((u, p, is_exit)) = stack.pop() {
            if is_exit {
                tout[u] = clock;
                continue;
            }
            clock += 1;
            tin[u] = clock;
            stack.push((u, p, true));
            for &v in &g[u] {
                if v == p {
                    continue;
                }
                depth[v] = depth[u] + 1;
                parent[v] = u;
                base_mask[v] = base_mask[u] ^ (1 << (s[v] - b'a'));
                stack.push((v, u, false));
            }
        }

        // ---------- 3. 倍增预处理 LCA ----------
        // 层级：log2(n) + 2 足够
        let log = (usize::BITS - n.leading_zeros()) as usize + 1;
        let mut up = vec![vec![0usize; n]; log];
        for v in 0..n {
            up[0][v] = parent[v];
        }
        for k in 1..log {
            for v in 0..n {
                up[k][v] = up[k - 1][up[k - 1][v]];
            }
        }

        let lca = |mut u: usize, mut v: usize| -> usize {
            if depth[u] < depth[v] {
                std::mem::swap(&mut u, &mut v);
            }
            // 先把 u 提升到与 v 同深度
            let diff = depth[u] - depth[v];
            for k in 0..log {
                if (diff >> k) & 1 == 1 {
                    u = up[k][u];
                }
            }
            if u == v {
                return u;
            }
            // 一起往上跳
            for k in (0..log).rev() {
                if up[k][u] != up[k][v] {
                    u = up[k][u];
                    v = up[k][v];
                }
            }
            up[0][u]
        };

        // ---------- 4. 处理查询 ----------
        let mut bit = Fenwick::new(n);
        let mut ch: Vec<u8> = s.to_vec();
        let mut ans = Vec::new();

        for q in &queries {
            let parts: Vec<&str> = q.split_whitespace().collect();
            if parts[0] == "update" {
                let u: usize = parts[1].parse().unwrap();
                let new_ch = parts[2].as_bytes()[0];

                let old_bit = 1 << (ch[u] - b'a');
                let new_bit = 1 << (new_ch - b'a');
                let delta = old_bit ^ new_bit;

                if delta != 0 {
                    // 子树区间整体异或 delta
                    bit.range_xor(tin[u], tout[u], delta);
                    ch[u] = new_ch;
                }
            } else {
                let u: usize = parts[1].parse().unwrap();
                let v: usize = parts[2].parse().unwrap();

                let w = lca(u, v);

                let mu = base_mask[u] ^ bit.point_query(tin[u]);
                let mv = base_mask[v] ^ bit.point_query(tin[v]);
                let mw = 1 << (ch[w] - b'a');

                let path_mask = mu ^ mv ^ mw;

                // 回文条件：最多一位为 1
                let ok = path_mask == 0 || (path_mask & (path_mask - 1)) == 0;
                ans.push(ok);
            }
        }

        ans
    }
}