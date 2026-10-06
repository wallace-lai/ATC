struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut v = Vec::with_capacity(s.len());

        for &c in s.as_bytes().iter() {
            if c == b'c' && v.len() >= 2 {
                let a = v[v.len() - 2];
                let b = v[v.len() - 1];
                if a == b'a' && b == b'b' {
                    v.pop();
                    v.pop();
                    continue;
                }
            }
            v.push(c);
        }

        v.len() == 0
    }
}