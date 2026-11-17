# 并查集 DSU

## 基本 DSU 

基本 DSU 模板，包含特性：

- 路径压缩；
- 按大小合并

适用场景：

- 判断连通性；
- 求连通分量数；
- 合并集合；

```rust
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

    // 迭代路径压缩，避免递归，完整路径压缩
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
```

## 按秩合并 DSU

选择建议：

- 只需要连通性，按大小合并（适用基本 DSU 即可）；
- 只需要树高上界（如可撤销 DSU），按秩合并；

```rust
pub struct DsuByRank {
    parent: Vec<usize>,
    rank: Vec<u32>,
}

impl DsuByRank {
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

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let (rx, ry) = (self.find(x), self.find(y));
        if rx == ry { return false; }

        if self.rank[rx] < self.rank[ry] {
            self.parent[rx] = ry;
        } else if self.rank[rx] > self.rank[ry] {
            self.parent[ry] = rx;
        } else {
            self.parent[ry] = rx;
            self.rank[rx] += 1;
        }
        true
    }
}
```

## 扩展域 / 种类并查集

适用场景：

- 判断“必须不同类”；
- 敌人的敌人是朋友；
- 二分图判定；
- A/B/C 三种关系的约束题；

具体做法：

- 为每个元素 `x` 开辟两倍空间，`x` 表示 `x` 属于 `A` 侧，`x + n` 表示 `x` 属于 `B` 侧；

- 约束 `x` 与 `y` 不同侧，则合并 `x` 与 `y + n`，以及 `x + n` 与 `y`；

```rust
// 扩展域 / 种类并查集（2n节点技巧）
pub struct DSU2n {
    parent: Vec<usize>,
    size: Vec<usize>,
    n: usize,
}

impl DSU2n {
    pub fn new(n: usize) -> Self {
        let m = 2 * n;
        Self {
            parent: (0..m).collect(),
            size: vec![1; m],
            n: n
        }
    }

    // 路径减半压缩
    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    fn union(&mut self, x: usize, y: usize) {
        let (mut rx, mut ry) = (self.find(x), self.find(y));
        if rx == ry { return; }

        if self.size[rx] < self.size[ry] {
            std::mem::swap(&mut rx, &mut ry);
        }
        self.parent[ry] = rx;
        self.size[rx] += self.size[ry];
    }

    // 断言 x 和 y 不同侧
    pub fn set_differenct(&mut self, x: usize, y: usize) {
        self.union(x, y + self.n);
        self.union(x + self.n, y);
    }

    // 断言 x 和 y 同侧
    pub fn set_same(&mut self, x: usize, y: usize) {
        self.union(x, y);
        self.union(x + self.n, y + self.n);
    }

    // 约束自相矛盾时返回 false
    pub fn consistent(&mut self, x: usize) -> bool {
        self.find(x) != self.find(x + self.n)
    }
}
```

## 带权 DSU（边权 = 相对关系）

适用场景：除了连通，还要回答两个点之间的：

- 相对差；
- 奇偶；
- 异或；
- 距离；

思想：`weight[x]`表示从 `x` 到 `parent[x]` 的一段“度量”，可以是差、乘积、异或等任何**可累积、可抵消**的运算，`find`时边权压缩把权重累加到根。

```rust
pub struct WeightedDSU {
    parent: Vec<usize>,
    rank: Vec<u32>,
    // weight[x] = value[x] - value[parent[x]]
    weight: Vec<i64>,
}

impl WeightedDSU {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            weight: vec![0; n],
        }
    }

    // 返回 x 的根，同时压缩路径并更新 weight[x] = value[x] - value[root]
    // 递归深度被压缩地极浅（一般 < 5），安全
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let p = self.parent[x];
            let root = self.find(p);
            // 先递归再累加，顺序不能反
            self.weight[x] += self.weight[p];
            self.parent[x] = root;
        }
        self.parent[x]
    }

    // 约束 value[x] - value[y] = w
    // 若与现有约束矛盾，返回 false
    pub fn union(&mut self, x: usize, y: usize, w: i64) -> bool {
        let (rx, ry) = (self.find(x), self.find(y));
        if rx == ry {
            return self.weight[x] - self.weight[y] == w;
        }

        let delta = w + self.weight[y] - self.weight[x];
        if self.rank[rx] < self.rank[ry] {
            // 反过来挂，权重取反
            self.parent[rx] = ry;
            self.weight[rx] = -delta;
        } else {
            self.parent[ry] = rx;
            self.weight[ry] = delta;
            if self.rank[rx] == self.rank[ry] {
                self.rank[rx] += 1;
            }
        }
        true
    }

    // 若 x 和 y 已连通，返回 value[x] - value[y]
    pub fn diff(&mut self, x: usize, y: usize) -> Option<i64> {
        let (rx, ry) = (self.find(x), self.find(y));
        if rx != ry { None } else { Some(self.weight[x] - self.weight[y])}
    }
}
```

## 可撤销 DSU

适用场景：需要撤销最近一次合并的场合。典型的是：

- 线段树分治 + 动态连通性；
- 回滚莫队
- 二分图动态判定

思路：为了能回滚，不能压缩路径，只能按秩/大小合并。用操作栈记录每次合并改动的字段。

```rust
pub struct RollbackDSU {
    parent: Vec<usize>,
    size: Vec<usize>,
    components: usize,
    // 历史记录
    // (child, parent_of_child_before) 和
    // (root, size_of_root_before)
    history: Vec<(usize, usize, usize, usize)>,
}

impl RollbackDSU {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            components: n,
            history: Vec::new(),
        }
    }

    /// 注意：不能路径压缩（否则回滚无法还原）
    pub fn find(&self, mut x: usize) -> usize {
        while self.parent[x] != x { x = self.parent[x]; }
        x
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let (mut rx, mut ry) = (self.find(x), self.find(y));
        if rx == ry { return false; }
        if self.size[rx] < self.size[ry] { std::mem::swap(&mut rx, &mut ry); }
        // 记录：child=ry, old_parent=ry, root=rx, old_size_of_root=size[rx]
        self.history.push((ry, ry, rx, self.size[rx]));
        self.parent[ry] = rx;
        self.size[rx] += self.size[ry];
        self.components -= 1;
        true
    }

    /// 回滚到快照（历史长度）对应的状态
    pub fn snapshot(&self) -> usize { self.history.len() }

    pub fn rollback(&mut self, snap: usize) {
        while self.history.len() > snap {
            let (child, old_parent, root, old_size) = self.history.pop().unwrap();
            self.parent[child] = old_parent;
            self.size[root] = old_size;
            self.components += 1;
        }
    }
}
```

## 稀疏/大值域 DSU（HashMap版）

适用场景：节点编号稀疏且范围巨大，或键是坐标、字符串、元组

```rust
use std::collections::HashMap;

pub struct SparseDSU<K: Eq + std::hash::Hash + Clone> {
    parent: HashMap<K, K>,
    size: HashMap<K, usize>,
}

impl<K: Eq + std::hash::Hash + Clone> SparseDSU<K> {
    pub fn new() -> Self {
        Self { parent: HashMap::new(), size: HashMap::new() }
    }

    pub fn add(&mut self, x: K) {
        if !self.parent.contains_key(&x) {
            self.parent.insert(x.clone(), x.clone());
            self.size.insert(x, 1);
        }
    }

    pub fn find(&mut self, x: &K) -> K {
        let p = self.parent[x].clone();
        if &p == x { return p; }
        let root = self.find(&p);
        self.parent.insert(x.clone(), root.clone());
        root
    }

    pub fn union(&mut self, x: &K, y: &K) -> bool {
        self.add(x.clone());
        self.add(y.clone());
        let (rx, ry) = (self.find(x), self.find(y));
        if rx == ry { return false; }
        let (sx, sy) = (self.size[&rx], self.size[&ry]);
        if sx < sy {
            self.parent.insert(rx.clone(), ry.clone());
            self.size.insert(ry.clone(), sx + sy);
        } else {
            self.parent.insert(ry.clone(), rx.clone());
            self.size.insert(rx, sx + sy);
        }
        true
    }
}
```
