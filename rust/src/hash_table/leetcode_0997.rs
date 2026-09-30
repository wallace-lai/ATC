struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut m = HashMap::with_capacity(trust.len());
        for t in trust {
            m.entry(t[0]).or_insert(vec![]).push(t[1]);
        }

        let mut no_trust = -1;
        for id in 1..=n {
            if !m.contains_key(&id) {
                if no_trust > -1 {
                    return -1;
                }
                no_trust = id;
            }
        }

        for id in 1..=n {
            if id == no_trust { continue; }
            if !m.contains_key(&id) { return -1; }
            let v = m.get(&id).unwrap();
            if !v.contains(&no_trust) { return -1; }
        }

        no_trust
    }
}