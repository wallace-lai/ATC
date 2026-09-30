struct Solution;

impl Solution {
    pub fn count_time(time: String) -> i32 {
        // H1H2:M1M2
        let b = time.as_bytes();
        let mut ans = 1;

        match (b[0], b[1]) {
            // H1和H2均未知
            (b'?', b'?') => {
                // H1=0, 1, H2=0~9
                // H1=2, H2=0, 1, 2, 3
                ans = 2 * 10 + 1 * 4;
            },
            // H1未知
            (b'?', _) => {
                // H2>=4, H1=0, 1
                // H2<4, H1=0, 1, 2
                ans = if b[1] >= b'4' { 2 } else { 3 };
            },
            // H2未知
            (_, b'?') => {
                // H1<2, H2=0~9
                // H2=2, H2=0, 1, 2, 3
                ans = if b[0] < b'2' { 10 } else { 4 };
            },
            // H1和H2均已知
            _ => {}
        }

        if b[3] == b'?' { ans *= 6; }
        if b[4] == b'?' { ans *= 10; }

        ans
    }
}