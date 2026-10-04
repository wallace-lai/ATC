struct Solution;

impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        let n = s.len();
        if n & 1 == 1 { return false; }
        let bs = s.as_bytes();
        let bl = locked.as_bytes();

        let mut min = 0;
        let mut max = 0;
        for i in 0..n {
            if bl[i] == b'1' {  // 无法修改s[i]
                let d = if bs[i] == b'(' { 1 } else { -1 };
                min += d;
                max += d;
                if max < 0 { return false; }
            } else {        // 可以修改s[i]
                min -= 1;   // 改成右括号，c减1
                max += 1;   // 改成左括号，c加1
            }
            if min < 0 { min = 1; }
        }

        min == 0
    }
}