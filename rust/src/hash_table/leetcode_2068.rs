struct Solution;

impl Solution {
    pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
        let mut count1 = [0; 26];
        let mut count2 = [0; 26];
        for &c in word1.as_bytes() {
            let idx = (c - b'a') as usize;
            count1[idx] += 1;
        }
        for &c in word2.as_bytes() {
            let idx = (c - b'a') as usize;
            count2[idx] += 1;
        }

        for i in 0..26 {
            let lhs: i32 = count1[i];
            let rhs: i32 = count2[i];
            if lhs.abs_diff(rhs) > 3 {
                return false;
            }
        }

        true
    }
}