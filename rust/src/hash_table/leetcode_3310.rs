struct Solution;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    // 有待优化，击败0%
    pub fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
        // 建图
        let mut indeg: Vec<i32> = vec![0; n as usize];
        let mut m: HashMap<i32, Vec<i32>> = HashMap::with_capacity(invocations.len());
        for edge in invocations.iter() {
            // edge[0] --> edge[1]
            indeg[edge[1] as usize] += 1;
            m.entry(edge[0]).or_insert(Vec::new()).push(edge[1]);
        }

        // println!("m is {:?}", m);
        println!("indeg is {:?}", indeg);

        // 用BFS找所有可疑方法
        let mut queue: VecDeque<i32> = VecDeque::with_capacity(invocations.len());
        let mut shady: HashSet<i32> = HashSet::with_capacity(invocations.len());
        shady.insert(k);
        queue.push_back(k);

        while !queue.is_empty() {
            let curr = queue.pop_front().unwrap();      // queue非空
            if let Some(next) = m.get(&curr) {
                for &i in next {
                    // curr --> i
                    indeg[i as usize] -= 1;

                    if !shady.contains(&i) {
                        shady.insert(i);
                        queue.push_back(i);
                    }
                }
            }
        }

        // 所有方法都是可疑方法，全部移除
        if shady.len() == n as usize {
            return vec![];
        }

        println!("shady is {:?}", shady);
        println!("indeg is {:?}", indeg);

        let mut ans: HashSet<i32> = (0..n).into_iter().collect();
        let mut indeg_sum = 0;
        for &i in shady.iter() {
            indeg_sum += indeg[i as usize];
        }
        // 可疑方法没有被非可疑方法依赖，可全部移除
        if indeg_sum == 0 {
            ans = &ans - &shady;
        }

        ans.into_iter().collect()
    }
}