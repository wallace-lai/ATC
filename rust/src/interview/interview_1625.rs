use std::collections::HashMap;

struct Node {
    key: i32,
    value: i32,
    prev: usize,
    next: usize,
}

struct LRUCache {
    map: HashMap<i32, usize>,   // key --> 节点在 nodes 数组中的索引
    nodes: Vec<Node>,           // 节点池，索引 0 和 1 为头尾哨兵
    size: usize,
    capacity: usize,
}

impl LRUCache {

    fn new(capacity: i32) -> Self {
        let cap = capacity as usize;
        let mut nodes = Vec::with_capacity(cap + 2);
        // 放置头尾哨兵
        nodes.push(Node { key: 0, value: 0, prev: 0, next: 1 });
        nodes.push(Node { key: 0, value: 0, prev: 0, next: 1 });

        Self {
            map: HashMap::new(),
            nodes: nodes,
            size: 0,
            capacity: cap,
        }
    }
    
    fn add_to_head(&mut self, idx: usize) {
        let first = self.nodes[0].next;
        self.nodes[0].next = idx;

        self.nodes[idx].prev = 0;
        self.nodes[idx].next = first;
        self.nodes[first].prev = idx;
    }

    fn remove_node(&mut self, idx: usize) {
        let prev = self.nodes[idx].prev;
        let next = self.nodes[idx].next;
        self.nodes[prev].next = next;
        self.nodes[next].prev = prev;
    }

    fn move_to_head(&mut self, idx: usize) {
        self.remove_node(idx);
        self.add_to_head(idx);
    }

    fn remove_tail(&mut self) -> usize {
        let idx = self.nodes[1].prev;
        self.remove_node(idx);
        idx
    }

    fn get(&mut self, key: i32) -> i32 {
        match self.map.get(&key) {
            Some(&idx) => {
                self.move_to_head(idx);
                self.nodes[idx].value
            },
            None => -1,
        }
    }
    
    fn put(&mut self, key: i32, value: i32) {
        if let Some(&idx) = self.map.get(&key) {
            // 如果 key 已存在，则将节点移动到头节点并更新 value
            self.nodes[idx].value = value;
            self.move_to_head(idx);
        } else {
            let idx = self.nodes.len();
            self.nodes.push(Node { key, value, prev: 0, next: 0 });
            self.map.insert(key, idx);
            self.add_to_head(idx);
            self.size += 1;

            if self.size > self.capacity {
                let tail_idx = self.remove_tail();
                let removed_key = self.nodes[tail_idx].key;
                self.map.remove(&removed_key);
                self.size -= 1;
                // 节点池中保留该节点（可进一步用空闲列表复用索引）
            }
        }
    }
}
