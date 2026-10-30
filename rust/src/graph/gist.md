# 图论

## 单源最短路径 - Dijkstra算法

定义：

Dijkstra 算法是一种用于求单源最短路径的经典贪心算法：给定一个带权图和一个起点 s ，它能算出从 s 到其他所有顶点的最短路径长度（也可以记录路径）。


思路：

维护每个顶点当前已知的**从起点到它的最短距离**：

1. 初始化：起点距离为 0 ，其他顶点距离为无穷大；
2. 每次从尚未确定最短距离的顶点中，选出当前距离最小的顶点 u；
3. 把 u 的最短距离视为已确定；
4. 用 u 去“松弛”它的邻居 v；
   1. 如果 dist[u] + w(u, v) < dist[v]，就更新 dist[v]
5. 重复直到所有可达顶点都被处理；

模板 - 只维护最短距离：

```rust
// 邻接表：adj[u] = Vec<(v, w)>
fn dijkstra(adj: &[Vec<(usize, u64)>], start: usize) -> Vec<u64> {
    const INF: u64 = u64::MAX;
    let n = adj.len();
    let mut dist = vec![INF; n];
    let mut heap = BinaryHeap::new();

    // 将起点入堆
    dist[start] = 0;
    // (d, u) - d 为从起点到 u 的最短距离，u 为当前遍历的节点
    heap.push(Reverse((0, start)));

    while let Some(Reverse((d, u))) = heap.pop() {
        // 懒删除：堆里可能有过时的条目
        if d > dist[u] { continue; }

        for &(v, w) in &adj[u] {
            let nd = d + w;
            if nd < dist[v] {
                dist[v] = nd;
                heap.push(Reverse((nd, v)));
            }
        }
    }

    dist
}
```

模板 - 维护最短距离和路径：

```rust
fn dijkstra_with_path(
    adj: &[Vec<(usize, u64)>],
    start: usize
) -> (Vec<u64>, Vec<Option<usize>>) {
    const INF: u64 = u64::MAX;
    let n = adj.len();
    let mut dist = vec![INF; n];
    // prev[i] 用于记录到达节点 i 的父节点
    let mut prev = vec![None; n];
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(Reverse((0, start)));

    while let Some(Reverse((d, u))) = heap.pop() {
        if d > dist[u] { continue; }

        for &(v, w) in &adj[u] {
            let nd = d + w;
            if nd < dist[v] {
                dist[v] = nd;
                prev[v] = Some(u);
                heap.push(Reverse((nd, v)));
            }
        }
    }

    (dist, prev)
}

// v 是终点
fn build_path(prev: &[Option<usize>], mut v: usize) -> Vec<usize> {
    let mut path = vec![];
    loop {
        path.push(v);
        match prev[v] {
            Some(p) => { v = p },
            None => { break },
        }
    }
    path.reverse();
    path
}
```

## 分层图最短路径（状态空间扩展）

```rust
// leetcode_3970.rs
// 法一：状态空间扩展 / 分层图最短路径
// 扩展 Dijstra 算法中的状态定义，将状态扩展为二元组 (u, 以节点u结尾的连续相同字符数c)
pub fn shortest_path(n: i32, edges: Vec<Vec<i32>>, labels: String, k: i32) -> i32 {
    const INF: u64 = u64::MAX;
    let n = n as usize;
    // 一条简单路径最多经过 n 个节点，连续相同字符数最多为 n
    let k = (k as usize).min(n);
    let labels = labels.as_bytes();

    // 构建邻接表
    let mut adj = vec![vec![]; n];
    for e in edges.iter() {
        let (u, v, w) = (e[0] as usize, e[1] as usize, e[2] as u64);
        adj[u].push((v, w));
    }

    // dist[u][c] - 到达节点 u 且路径末尾恰好有 c 个连续相同字符时的最小总边权
    let mut dist = vec![vec![INF; k + 1]; n];
    let mut heap = BinaryHeap::new();

    // 起点入堆
    dist[0][1] = 0;
    heap.push(Reverse((0, 0, 1)));  // (距离，节点，连续字符数)

    while let Some(Reverse((d, u, c))) = heap.pop() {
        if d > dist[u][c] { continue; }
        if u == n - 1 { return d as i32; }

        for &(v, w) in &adj[u] {
            let next_c = if labels[v] == labels[u] { c + 1 } else { 1 };
            if next_c > k { continue; } // 连续字符数大于 K，剪枝

            let nd = d + w;
            if nd < dist[v][next_c] {
                dist[v][next_c] = nd;
                heap.push(Reverse((nd, v, next_c)));
            }
        }
    }

    // 在终点 n-1 的所有合法连续数中取最小值
    let ans = (1..=k).map(|k| dist[n - 1][k]).min().unwrap();
    if ans == INF { -1 } else { ans as i32 }
}
```