struct Solution;

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let b = s.as_bytes();
        let n = b.len();
        let mut min = 0;
        let mut max = 0;

        for i in 0..n {
            if b[i] == b'(' {
                min += 1;
                max += 1;
            } else if b[i] == b')' {
                min -= 1;
                max -= 1;
                if max < 0 {    // 右括号太多了
                    return false;
                }
            } else {    // b[i]为 '*' 号，可以修改
                min -= 1;   // '*'号改成右括号
                max += 1;   // '*'号改成左括号
            }
            min = min.max(0);   // 未匹配的左括号个数不能为负数
        }

        min == 0
    }
}