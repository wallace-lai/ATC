struct Solution;

#[derive(Default)]
struct Node {
    // 关键限定：
    // 1. 单词只包含26个小写英文字母
    // 2. next[i] == 0 表示无子节点（0号是根，真实节点索引恒 >= 1）
    next: [u32; 26],    // 存储子节点索引号，0 表示空
    count: u32,         // 以当前字母结束的单词数量
    index: i32,         // 单词在 small 中的下标，-1 表示非单词结尾
}

pub struct Trie {
    nodes: Vec<Node>,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            nodes: vec![Node {
                index: -1,
                ..Default::default()
            }],
        }
    }
    
    // 字节 -> 0..26
    #[inline]
    fn index(c: u8) -> Option<usize> {
        // b < 'a' 时回绕成最大值，被下一步排除
        let i = c.wrapping_sub(b'a');
        (i < 26).then_some(i as usize)
    }

    // 插入时额外记录单词在 small 中的id
    pub fn insert_with_id(&mut self, word: &str, id: usize) {
        let mut curr = 0;
        for &c in word.as_bytes() {
            let i = Self::index(c).expect("word must contain only 'a'..='z' !");
            let next = self.nodes[curr].next[i];
            curr = if next != 0 {
                next as usize
            } else {
                let n = self.nodes.len();
                self.nodes.push(Node {
                    index: -1,
                    ..Default::default()
                });
                self.nodes[curr].next[i] = n as u32;
                n
            };
        }
        self.nodes[curr].count += 1;
        self.nodes[curr].index = id as i32;
    }

    // 插入单词，重复插入会累加 count
    // 约束：word 只允许小写字母 a - z，否则 panic
    pub fn insert(&mut self, word: &str) {
        let mut curr = 0;
        for &c in word.as_bytes() {
            let i = Self::index(c).expect("word must contain only 'a'..='z' !");
            let next = self.nodes[curr].next[i];
            curr = if next != 0 {
                next as usize
            } else {
                let n = self.nodes.len();
                self.nodes.push(Node::default());
                self.nodes[curr].next[i] = n as u32;
                n
            };
        }
        self.nodes[curr].count += 1;
    }

    // 搜索完整单词是否存在（任意次插入后 count > 0）
    pub fn search(&self, word: &str) -> bool {
        self.walk(word).map_or(false, |n| self.nodes[n].count > 0)
    }

    // 是否存在以 prefix 为前缀的已插入单词
    pub fn starts_with(&self, prefix: &str) -> bool {
        self.walk(prefix).is_some()
    }

    // 可选：查询某单词出现的次数
    pub fn count_of(&self, word: &str) -> u32 {
        self.walk(word).map_or(0, |n| self.nodes[n].count)
    }

    // 可选：删除一次单词（count归零后不再视为单词，但路径保留）
    pub fn delete(&mut self, word: &str) -> bool {
        match self.walk(word) {
            Some(n) if self.nodes[n].count > 0 => {
                self.nodes[n].count -= 1;
                // 返 true 表示确实进行了删除，否则表示所删单词不存在
                true
            },
            _ => false,
        }
    }

    // 沿字节路径走到底，返回终止节点索引
    fn walk(&self, s: &str) -> Option<usize> {
        let mut curr = 0;
        for &c in s.as_bytes() {
            let i = Self::index(c)?;
            let next = self.nodes[curr].next[i];
            if next == 0 {
                return None;
            }
            curr = next as usize;
        }
        Some(curr)
    }

    // 从 big 的 start 位置出发，在 Trie 上匹配
    // 将命中单词的起始位置 start 推入 ans[id] 中
    pub fn match_from(&self, big: &[u8], start: usize, ans: &mut [Vec<usize>]) {
        let mut curr = 0;
        for pos in start..big.len() {
            let i = match Self::index(big[pos]) {
                Some(i) => i,
                None => return, // 非法字符，直接结束
            };

            let next = self.nodes[curr].next[i];
            if next == 0 { return; }    // Trie 中无此路径，剪枝结束

            curr = next as usize;
            // 如果当前节点是否个单词的结尾，记录起始位置
            if self.nodes[curr].index >= 0 {
                ans[self.nodes[curr].index as usize].push(start);
            }
        }
    }
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}

impl Solution {
    // 法一：字典树
    pub fn multi_search(big: String, smalls: Vec<String>) -> Vec<Vec<i32>> {
        let mut trie = Trie::new();
        for (i, w) in smalls.iter().enumerate() {
            trie.insert_with_id(w, i);
        }

        let bytes = big.as_bytes();
        let mut ans: Vec<Vec<usize>> = vec![vec![]; smalls.len()];

        // 从 big 的每个位置出发，在 Trie 上匹配
        for start in 0..bytes.len() {
            trie.match_from(bytes, start, &mut ans);
        }

        // 转换
        ans.into_iter()
            .map(|v| v.into_iter().map(|x| x as i32).collect())
            .collect()
    }
}