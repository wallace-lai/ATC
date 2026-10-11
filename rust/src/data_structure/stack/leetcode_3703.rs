struct Solution;

impl Solution {
    // k = 1
    pub fn remove_substring_simple(s: String) -> String {
        let mut stk = Vec::with_capacity(s.len());
        for &c in s.as_bytes().iter() {
            if c == b')' && stk.len() > 0 && stk[stk.len() - 1] == b'(' {
                stk.pop();
                continue;
            }
            stk.push(c);
        }
        unsafe { String::from_utf8_unchecked(stk) }
    }

    pub fn remove_substring(s: String, k: i32) -> String {
        if k == 1 { return Self::remove_substring_simple(s); }
        let mut stk = Vec::new();

        for &c in s.as_bytes().iter() {
            if c == b')' && stk.len() >= 2 {
                let n = stk.len();
                let t1: (u8, i32) = stk[n - 1];
                let t2: (u8, i32) = stk[n - 2];
                if t2.0 == b'(' && t1.0 == b')' &&
                    t2.1 >= k && t1.1 == k - 1 {
                    let r = t2.1 - k;
                    stk.pop();
                    stk.pop();
                    if r > 0 {
                        stk.push((b'(', r));
                    }
                    continue;
                }
            }

            if stk.len() == 0 {
                stk.push((c, 1));
            } else {
                let top = stk.last_mut().unwrap();
                if top.0 == c { top.1 += 1; } else { stk.push((c, 1)); }
            }
        }

        let mut v = Vec::with_capacity(s.len());
        for (c, n) in stk {
            for _ in 0..n {
                v.push(c);
            }
        }

        unsafe { String::from_utf8_unchecked(v) }
    }
}