struct Solution;

impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        fn atoi(s: &str) -> i32 {
            let b = s.as_bytes();
            let n = b.len();

            let mut i = 0;
            let mut ans = 0i64;
            let mut sign = None;

            while i < n {
                let c = b[i] as char;
                if c.is_ascii_whitespace() {
                    // 忽略前导空格，如果出现正负号以后再次出现空格，则表达式非法
                    if sign.is_some() { break; }
                    i += 1;
                    continue;
                }
                
                if c == '+' || c == '-' {
                    // 遇到正负号
                    if sign.is_some() { break; }
                    sign = Some(c);
                    i += 1;
                } else if c.is_ascii_digit() {
                    // 遇到数字
                    let mut p = i;
                    let mut num = 0i64;
                    while p < n && b[p].is_ascii_digit() {
                        num = num * 10 + (b[p] - b'0') as i64;
                        if num > i32::MAX as i64 { break; }
                        p += 1;
                    }
                    ans = if sign == Some('-') { -num } else { num };
                    break;
                } else {
                    // 遇到非法字符
                    break;
                }
            }

            if ans < i32::MIN as i64 {
                i32::MIN
            } else if ans > i32::MAX as i64 {
                i32::MAX
            } else {
                ans as i32
            }
        }

        atoi(str.as_str())
    }
}
