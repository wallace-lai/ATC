struct Solution;

impl Solution {
    pub fn reverse_degree(s: String) -> i32 {
        let mut deg = 0;
        for (i, &c) in s
            .as_bytes()
            .iter()
            .enumerate() {
            deg += (26 - (c - b'a')) as i32 * (i as i32 + 1);
        }

        deg
    }
}