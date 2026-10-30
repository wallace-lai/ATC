struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
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
}
