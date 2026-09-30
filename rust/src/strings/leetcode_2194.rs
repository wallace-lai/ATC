struct Solution;

impl Solution {
    pub fn cells_in_range(s: String) -> Vec<String> {
        // "K1:L2"
        let b = s.as_bytes();
        let c1 = b[0];
        let c2 = b[3];
        let r1 = b[1];
        let r2 = b[4];

        let mut ans = Vec::new();
        for c in c1..=c2 {
            for r in r1..=r2 {
                let cell = vec![c, r];
                ans.push(unsafe {
                    String::from_utf8_unchecked(cell)
                });
            }
        }

        ans
    }
}