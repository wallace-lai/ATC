struct Solution;

impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let mut ans = 0;
        let mut prev = None;

        for i in 0..32 {
            if (n >> i) & 1 == 1 {
                if prev.is_some() {
                    let d = i - prev.unwrap();
                    ans = ans.max(d);
                }
                prev = Some(i);
            }
        }

        ans
    }
}