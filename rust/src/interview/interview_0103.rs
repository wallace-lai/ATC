struct Solution;

impl Solution {
    pub fn replace_spaces(mut s: String, length: i32) -> String {
        // 1. 先确定有多少个空格
        let n = length as usize;
        let slice = &s.as_bytes()[0..n];
        let space = slice.iter()
            .filter(|&c| c.is_ascii_whitespace())
            .count();

        // 2. URL化之后的字符串实际长度（每个空格多占据2个字符）
        let len = n + space * 2;
        
        // 3. 原地修改
        unsafe {
            let b = s.as_mut_vec();
            let mut r = length - 1;   // 读指针
            let mut w = len - 1;    // 写指针
            while r >= 0 {
                let c = b[r as usize];
                if c == b' ' {
                    b[w] = b'0'; w -= 1;
                    b[w] = b'2'; w -= 1;
                    b[w] = b'%'; w -= 1;
                } else {
                    b[w] = c; w -= 1;
                }
                r -= 1;
            }
            b.truncate(len);
            s
        }
    }
}