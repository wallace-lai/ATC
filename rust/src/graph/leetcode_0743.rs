struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    // 法一： Dijstra算法
    // 思路：给定一张非负权有向图和起点 K，求从 K 到所有顶点的单源最短路径
    // 如果存在某个 dist[i] == INF，说明该节点不可达，返 -1
    // 否则答案就是 max(dist[1..=n])
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;

        // 构建邻接表 adj[u] = Vec<(v, w)>
        let mut adj = vec![vec![]; n + 1];
        for t in times.iter() {
            let (u, v, w) = (t[0] as usize, t[1] as usize, t[2] as u64);
            adj[u].push((v, w));
        }

        const INF: u64 = u64::MAX;
        let mut dist = vec![INF; n + 1];
        let mut heap = BinaryHeap::new();

        dist[k] = 0;
        heap.push(Reverse((0, k))); // (距离，节点)

        while let Some(Reverse((d, u))) = heap.pop() {
            if d > dist[u] { continue; }

            for &(v, w) in &adj[u] {
                let nd = d + w;
                if nd < dist[v] {
                    dist[v] = nd;
                    heap.push(Reverse((nd, v)));
                }
            }
        }

        // 收集答案
        let mut ans = 0;
        for i in 1..=n {
            if dist[i] == INF {
                return -1;
            }
            ans = ans.max(dist[i]);
        }

        ans as i32
    }
}