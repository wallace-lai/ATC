struct Solution;

#[cfg(any())]
impl Solution {
    // 法一：动态规划 + 哈希表
    // 运行时间：174ms
    pub fn respace(dictionary: Vec<String>, sentence: String) -> i32 {
        let n = sentence.len();
        // 哈希表：便于判断子串是否存在
        let dict: HashSet<&str> = dictionary.iter().map(|s| s.as_str()).collect();
        // dp[i] 表示前 i 个字符中最少未被识别的字符数
        let mut dp = vec![0; n + 1];

        for i in 1..=n {
            // 情况1：第 i 个字符单独无法识别
            dp[i] = dp[i - 1] + 1;

            // 情况2：枚举所有以 i 结尾的子串，若在词典中则完全可以识别
            for j in 0..i {
                if dict.contains(&sentence[j..i]) {
                    dp[i] = dp[i].min(dp[j]);
                    // dp[i] 已经为0，不可能更小，提前退出
                    if dp[i] == 0 { break; }
                }
            }
        }

        dp[n]
    }
}

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

    // 从节点 node 沿字符 c 走一步；无对应子节点返回 None
    // 非小写字母（如空格）会经由 Self::index 返回 None
    #[inline]
    pub fn child(&self, node: usize, c: u8) -> Option<usize> {
        let i = Self::index(c)?;
        let next = self.nodes[node].next[i];
        (next != 0).then_some(next as usize)
    }

    // 节点 node 是否是某个已插入单词的结尾
    #[inline]
    pub fn is_word(&self, node: usize) -> bool {
        self.nodes[node].count > 0
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
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}

impl Solution {
    // 法二：动态规划 + 反向字典树
    // 运行时间：11ms
    pub fn respace(dictionary: Vec<String>, sentence: String) -> i32 {
        let mut trie = Trie::new();

        // 反向插入词典
        for w in dictionary.iter() {
            let rev: String = w.chars().rev().collect();
            trie.insert(&rev);
        }

        let s = sentence.as_bytes();
        let n = s.len();
        // dp[i] 表示前 i 个字符中最少未识别的字符数
        let mut dp = vec![0; n + 1];

        for i in 1..=n {
            // 默认第 i 个字符无法识别
            dp[i] = dp[i - 1] + 1;

            // 从根出发
            let mut curr = 0;
            // 从 i 往前倒序，刚好对应反序插入的字典树正向路径
            for j in (1..=i).rev() {
                match trie.child(curr, s[j - 1]) {
                    Some(next) => {
                        curr = next;
                        if trie.is_word(curr) {
                            // sentence[j-1..i] 是一个完整单词
                            dp[i] = dp[i].min(dp[j - 1]);
                        }
                    },
                    None => break,  // 失配剪枝：空格也会走到这里直接break
                }
                // dp[i] 已经为0，不可能更小，提前退出
                if dp[i] == 0 { break; }
            }
        }

        dp[n] as _
    }
}