struct Solution;

impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut v = vec![first; encoded.len() + 1];
        for i in 1..v.len() {
            v[i] = encoded[i - 1] ^ v[i - 1];
        }
        v
    }
}