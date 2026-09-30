struct Solution;

impl Solution {
    pub fn buddy_strings(s: String, g: String) -> bool {
        // 情况一：两字符串长度不相等
        if s.len() != g.len() { return false; }

        // 情况二：两字符串相等，查看是否有重复字符
        if s == g {
            let mut count = [0; 26];
            for &c in s.as_bytes() {
                let idx = (c - b'a') as usize;
                // 存在重复字符
                if count[idx] == 1 { return true; }
                count[idx] += 1;
            }
            return false;
        }

        // 情况三：两字符长度相等但本身不相等
        let mut s_char = Vec::with_capacity(4);
        let mut g_char = Vec::with_capacity(4);
        for i in 0..s.len() {
            if s.as_bytes()[i] != g.as_bytes()[i] {
                if s_char.len() >= 2 { return false; }
                s_char.push(s.as_bytes()[i]);
                g_char.push(g.as_bytes()[i]);
            }
        }
        if s_char.len() != 2 { return false; }
        s_char[0] == g_char[1] && s_char[1] == g_char[0]
    }
}