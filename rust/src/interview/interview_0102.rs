struct Solution;

impl Solution {
    pub fn check_permutation(s1: String, s2: String) -> bool {
        let mut v1 = [0u8; 26];
        let mut v2 = [0u8; 26];

        fn count(s: &str, v: &mut [u8]) {
            for &c in s.as_bytes() {
                let i = (c - b'a') as usize;
                v[i] += 1;
            }
        }
        count(&s1, &mut v1);
        count(&s2, &mut v2);

        v1 == v2
    }
}