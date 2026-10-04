struct Solution;

impl Solution {
    pub fn robot_with_string(s: String) -> String {
        let b = s.as_bytes();
        let n = b.len();

        let mut count = [0; 26];
        for &c in s.as_bytes().iter() {
            let idx = (c - b'a') as usize;
            count[idx] += 1;
        }

        let mut pos = 0;
        let mut stk = Vec::with_capacity(n);
        let mut ans = String::with_capacity(n);

        // 从a..z中枚举当前要找的字符 curr
        for i in 0..count.len() {
            let curr = (i as u8 + b'a') as char;

            // 如果栈中暂存的字符比要找的字符还小，则需要先退栈
            while stk.len() > 0 && stk[stk.len() - 1] <= curr {
                ans.push(stk[stk.len() - 1]);
                stk.pop();
            }

            // 从s中剩余的子串（s[pos..]）中查找
            while count[i] > 0 && pos < n {
                // 每当从s中遍历到一个字符，计数器都需要减去1
                let idx = (b[pos] - b'a') as usize;
                count[idx] -= 1;

                // 如果遍历到的字符不是当前要找的字符curr，则入栈否则写入答案
                if b[pos] as char != curr {
                    stk.push(b[pos] as char);
                } else {
                    ans.push(b[pos] as char);
                }

                pos += 1;
            }
        }

        // 将栈中的字符全部退栈
        ans.extend(stk.iter().rev());
        ans
    }
}