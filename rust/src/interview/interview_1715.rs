struct Solution;

#[derive(Default)]
struct Node {
    // 关键限定：
    // 1. 单词只包含26个小写英文字母
    // 2. next[i] == 0 表示无子节点（0号是根，真实节点索引恒 >= 1）
    next: [u32; 26],    // 存储子节点索引号，0 表示空
    count: u32,         // 以当前字母结束的单词数量
}

pub struct Trie {
    nodes: Vec<Node>,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            nodes: vec![Node::default()]    // 0 号节点为根
        }
    }
    
    // 字节 -> 0..26
    #[inline]
    fn index(c: u8) -> Option<usize> {
        // b < 'a' 时回绕成最大值，被下一步排除
        let i = c.wrapping_sub(b'a');
        (i < 26).then_some(i as usize)
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

    // 判断 bytes[start..] 能否被拆成至少 1 个词典中的单词
    // memo[start] = 
    // 0  -> 未计算
    // 1  -> 可以
    // -1 -> 不可以
    fn can_break_from(&self, bytes: &[u8], start: usize, memo: &mut [i8]) -> bool {
        let n = bytes.len();
        if start >= n { return false; }     // 空串不算一个单词
        if memo[start] != 0 { return memo[start] == 1; }

        let mut curr = 0;   // 每次都从根开始
        for pos in start..n {
            let i = match Self::index(bytes[pos]) {
                Some(i) => i,
                None => {
                    // 遇到非小写字母
                    memo[start] = -1;
                    return false;
                }
            };

            let next = self.nodes[curr].next[i];
            if next == 0 {
                memo[start] = -1;
                return false;
            }

            curr = next as usize;

            // 在 pos 处命中一个单词
            if self.nodes[curr].count > 0 {
                if pos == n - 1 {
                    // 剩余部分刚好是一个单词
                    memo[start] = 1;
                    return true;
                }
                // 继续分割剩余部分
                if self.can_break_from(bytes, pos + 1, memo) {
                    memo[start] = 1;
                    return true;
                }
            }
        }

        memo[start] = -1;
        false
    }

    // 判断 word 能否被拆成至少 2 个词典中的单词（单词可重复使用）
    pub fn can_break_into_two_or_more(&self, word: &str) -> bool {
        let bytes = word.as_bytes();
        let n = bytes.len();
        if n < 2 { return false; }

        let mut memo = vec![0i8; n];
        let mut curr = 0usize;

        // pos 范围 0..=n-2
        for pos in 0..(n - 1) {
            let i = match Self::index(bytes[pos]) {
                Some(i) => i,
                None => return false,
            };

            let next = self.nodes[curr].next[i];
            if next == 0 { return false; }
            curr = next as usize;

            if self.nodes[curr].count > 0 {
                // word[0..=pos] 是一个单词，剩余 word[pos+1..] 需要能被拆分
                if self.can_break_from(bytes, pos + 1, &mut memo) {
                    return true;
                }
            }
        }

        false
    }

}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}

impl Solution {
    // 法一：DFS + 字典树
    pub fn longest_word(words: Vec<String>) -> String {
        let mut trie = Trie::new();
        for w in words.iter() {
            trie.insert(w);
        }
        
        // 排序策略：长度降序，字典序升序
        // 这样第一个满足条件的单词就是答案
        let mut sorted: Vec<&String> = words.iter().collect();
        sorted.sort_by(|a, b| b.len().cmp(&a.len()).then_with(|| a.cmp(b)));
        
        // 剪枝：最短单词长度
        let min_len = words.iter().map(|w| w.len()).min().unwrap_or(0);

        for w in sorted {
            // 连两个最短单词都拼不出来，直接跳过
            if w.len() < 2 * min_len { continue; }

            if trie.can_break_into_two_or_more(w) {
                return w.clone();
            }
        }
        
        String::new()
    }
}