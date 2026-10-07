struct Solution;

impl Solution {
    pub fn merge_characters(s: String, k: i32) -> String {
        let k = k as usize;
        let mut stks = vec![vec![]; 26];
        let mut flag = vec![true; s.len()];

        let mut m = 0;  // 记录发生的合并次数
        for (i, &c) in s.as_bytes().iter().enumerate() {
            let stk = &stks[(c - b'a') as usize];
            if stk.len() > 0 {
                // 减去m后才是合并后字符c的实际位置
                let d = i - m - stk[stk.len() - 1];
                if d <= k {
                    m += 1;
                    flag[i] = false;
                    continue;
                }
            }
            stks[(c - b'a') as usize].push(i - m);
        }

        s.as_bytes()
            .iter()
            .enumerate()
            .filter(|(i, _)| flag[*i] == true)
            .map(|(_, c)| *c as char)
            .collect()
    }
}