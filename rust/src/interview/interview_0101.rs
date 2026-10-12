struct Solution;

impl Solution {
    pub fn is_unique(astr: String) -> bool {
        if astr.len() > 26 { return false; }
        let mut mask = 0u32;

        for &c in astr.as_bytes() {
            let i = (c - b'a') as usize;
            if (mask >> i) & 1 == 1 {
                return false;
            }
            mask |= 1 << i;
        }

        true
    }
}