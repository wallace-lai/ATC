struct Solution;

use std::collections::HashMap;
use std::collections::VecDeque;

impl Solution {
    pub fn find_whether_exists_path(n: i32, graph: Vec<Vec<i32>>, start: i32, target: i32) -> bool {
        // (节点 --> 节点可通往的下一个节点列表)
        let mut m: HashMap<usize, Vec<usize>> = HashMap::new();
        for g in graph {
            if g[0] == g[1] { continue; }   // 排除自环
            m.entry(g[0] as usize).or_insert(Vec::new()).push(g[1] as usize);
        }

        let n = n as usize;
        let start = start as usize;
        let target = target as usize;
        if start == target { return true; }

        let mut vis = vec![false; n];
        let mut q = VecDeque::new();

        q.push_back(start);
        vis[start] = true;

        while !q.is_empty() {
            let len = q.len();
            for _ in 0..len {
                let curr = q.pop_front().unwrap();
                if let Some(v) = m.get(&curr) {
                    for &next in v {
                        // 跳过已经被访问过的节点，平行边也会被跳过
                        if vis[next] == true { continue; }
                        if next == target { return true; }
                        q.push_back(next);
                        vis[next] = true;
                    }
                }
            }
        }

        false
    }
}