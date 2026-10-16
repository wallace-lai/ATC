struct Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        let b = s.as_bytes();
        let n = b.len();
        let mut seg = Vec::with_capacity(n);
        let mut stk = Vec::with_capacity(n);

        for i in 0..n {
            if b[i] != b']' {
                stk.push(b[i]);
                continue;
            }

            seg.clear();
            while let Some(&t) = stk.last() {
                if t == b'[' {
                    stk.pop();
                    break;
                }
                seg.push(t);
                stk.pop();
            }

            let mut k = 0;
            let mut p = 1;
            while let Some(&t) = stk.last() {
                if !t.is_ascii_digit() {
                    break;
                }

                k += p * (t - b'0') as i32;
                p *= 10;
                stk.pop();
            }

            for _ in 0..k {
                for &c in seg.iter().rev() {
                    stk.push(c);
                }
            }
        }

        unsafe { String::from_utf8_unchecked(stk) }
    }
}