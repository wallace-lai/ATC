struct Solution;

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut b = s.into_bytes();
        let n = b.len();
        let mut stk = Vec::with_capacity(n);

        for i in 0..n {
            match b[i] {
                b')' => {
                    if let Some(&j) = stk.last() {
                        if b[j] == b'(' {
                            stk.pop();
                            continue;
                        }
                    }
                    stk.push(i);
                },
                b'(' => { stk.push(i); },
                _ => {}
            }
        }

        // 原地压缩
        let mut i = 0;
        let mut write = 0;
        for read in 0..n {
            if i < stk.len() && stk[i] == read {
                i += 1;
            } else {
                b[write] = b[read];
                write += 1;
            }
        }
        b.truncate(write);

        unsafe { String::from_utf8_unchecked(b) }
    }
}