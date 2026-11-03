struct Solution;

use std::collections::HashMap;

pub struct NameDsu {
    parent: HashMap<String, String>
}

impl NameDsu {
    fn new() -> Self {
        Self {
            parent: HashMap::new()
        }
    }

    fn add(&mut self, s: &str) {
        self.parent.entry(s.to_string()).or_insert_with(|| s.to_string());
    }

    // 带路径压缩的查找
    fn find(&mut self, s: &str) -> String {
        let key = s.to_string();
        let p = self.parent.get(&key)
            .cloned()
            .unwrap_or_else(|| key.clone());
        if p == key {
            return key;
        }

        let root = self.find(&p);
        self.parent.insert(key, root.clone());
        root
    }

    // 合并：字典序较小的名字作为根
    fn union(&mut self, a: &str, b: &str) {
        self.add(a);
        self.add(b);
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb {
            return;
        }

        if ra < rb {
            self.parent.insert(rb, ra);
        } else {
            self.parent.insert(ra, rb);
        }
    }
}

impl Solution {
    pub fn truly_most_popular(names: Vec<String>, synonyms: Vec<String>) -> Vec<String> {
        let mut dsu = NameDsu::new();
        let mut freq_map: HashMap<String, u64> = HashMap::new();

        for n in &names {
            let mut iter = n.split('(');
            let name = iter.next().unwrap();
            let freq_str = iter.next().unwrap().trim_end_matches(')');
            let freq: u64 = freq_str.parse().unwrap();

            dsu.add(name);
            *freq_map.entry(name.to_string()).or_insert(0) += freq;
        }

        for s in &synonyms {
            // "(Jon,John)" -> "Jon,John"
            let inner = &s[1..s.len() - 1];
            let mut iter = inner.split(',');
            let a = iter.next().unwrap();
            let b = iter.next().unwrap();
            dsu.union(a, b);
        }

        let mut result_map: HashMap<String, u64> = HashMap::new();
        for (name, freq) in freq_map {
            let root = dsu.find(&name);
            *result_map.entry(root).or_insert(0) += freq;
        }

        let mut result: Vec<(String, u64)> = result_map.into_iter().collect();
        result.sort();

        result.into_iter()
            .map(|(name, freq)| format!("{}({})", name, freq))
            .collect()
    }
}