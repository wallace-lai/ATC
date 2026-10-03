struct Solution;

impl Solution {
    // O（n|A| + nlogn)
    // 19ms，击败100%
    pub fn clear_stars(s: String) -> String {
        let b = s.as_bytes();
        let n = b.len();
        // 26个字母，每个对应一个栈
        let mut stks = vec![vec![]; 26];

        for (i, &c) in b.iter().enumerate() {
            if c != b'*' {
                let idx = (c - b'a') as usize;
                stks[idx].push(i);
                continue;
            }

            // c == b'*'，假设c最前面字典序最小的字符为x，则：
            // （1）如果c前面全为x，则删除离c最近的x是可以的；
            // （2）如果c前面有比x更大的字符y，为了避免删除x后y往前
            //  移动导致字典序变大，此时仍然应该删除离c最近的x；
            for stk in stks.iter_mut() {
                if stk.len() > 0 {
                    stk.pop();
                    break;
                }
            }
        }

        let mut idx = Vec::with_capacity(n);
        for stk in stks.iter() {
            idx.extend_from_slice(stk);
        }
        idx.sort_unstable();

        let mut ans = String::with_capacity(n);
        for i in idx {
            ans.push(b[i] as char);
        }

        ans
    }
}