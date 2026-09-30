struct Solution;

impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        let mut v: Vec<i32> = s.as_bytes()
            .iter()
            .map(|c| (*c - b'0') as i32)
            .collect();

        let mut t = Vec::with_capacity(v.len());
        while v.len() > 2 {
            t.clear();
            t.extend(v.windows(2).map(|w| (w[0] + w[1]) % 10));
            std::mem::swap(&mut v, &mut t);
        }

        v[0] == v[1]
    }
}