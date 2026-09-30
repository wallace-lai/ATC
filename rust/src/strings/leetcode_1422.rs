struct Solution;

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let n = s.len();
        let b = s.as_bytes();
        let mut pre = vec![0; n];
        let mut suf = vec![0; n];

        pre[0] = if b[0] == b'0' { 1 } else { 0 };
        for i in 1..n {
            pre[i] = if b[i] == b'0' { pre[i - 1] + 1 } else { pre[i - 1] };
        }

        suf[n - 1] = if b[n - 1] == b'1' { 1 } else { 0 };
        for i in (0..(n - 1)).rev() {
            suf[i] = if b[i] == b'1' { suf[i + 1] + 1 } else { suf[i + 1] };
        }

        // println!("pre is {:?}", pre);
        // println!("suf is {:?}", suf);

        let mut ans = 0;
        for i in 0..(n - 1) {
            ans = std::cmp::max(ans, pre[i] + suf[i + 1]);
        }

        ans
    }
}