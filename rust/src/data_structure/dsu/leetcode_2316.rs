struct Solution;

impl Solution {
    #[cfg(any())]
    // 法一：显式栈DFS
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;

        // 建图
        let mut g = vec![vec![]; n];
        for e in edges.iter() {
            g[e[0] as usize].push(e[1]);
            g[e[1] as usize].push(e[0]);
        }

        let mut ans = 0;
        let mut vis = vec![false; n];
        let mut stk = Vec::new();

        for i in 0..n {
            if vis[i] { continue; }

            let mut count = 0;

            stk.push(i);
            vis[i] = true;

            while let Some(x) = stk.pop() {
                count += 1;

                for &y in &g[x] {
                    let y = y as usize;
                    if !vis[y] {
                        vis[y] = true;
                        stk.push(y);
                    }
                }
            }

            ans += count * (n - count);
        }

        ans as i64 / 2
    }

    // 法二：并查集
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;
        let mut dsu = DSU::new(n);
        for e in edges {
            let (x, y) = (e[0] as usize, e[1] as usize);
            dsu.union(x, y);
        }

        let mut ans = 0;
        for i in 0..n {
            if dsu.find(i) == i {
                let s = dsu.size_of(i);
                ans += s * (n - s);
            }
        }

        ans as i64 / 2
    }
}

// 基本并查集（按大小合并）
pub struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,   // 根节点存分量大小
    components: usize,  // 当前连通分量个数
}

impl DSU {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            components: n
        }
    }

    // 迭代路径压缩，避免递归
    pub fn find(&mut self, mut x: usize) -> usize {
        let mut root = x;
        while self.parent[root] != root {
            root = self.parent[root];
        }

        // 路径压缩：把路径上的点直接挂到根节点
        while self.parent[x] != root {
            let next = self.parent[x];
            self.parent[x] = root;
            x = next;
        }

        root
    }

    // 返回 true 表示本次合并真的合并了两个不同的集合
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let (mut rx, mut ry) = (self.find(x), self.find(y));
        if rx == ry { return false; }

        if self.size[rx] < self.size[ry] {
            std::mem::swap(&mut rx, &mut ry);
        }
        self.parent[ry] = rx;
        self.size[rx] += self.size[ry];
        self.components -= 1;
        true
    }

    // 判断 x 和 y 是否相连（位于同一个连通分量中）
    pub fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    // 返回 x 所在集合的元素个数
    pub fn size_of(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root]
    }
}