struct Solution;

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        if s1 == s2 { return true; }

        let mut count1 = [0; 26];
        let mut count2 = [0; 26];
        for &c in s1.as_bytes() {
            count1[(c - b'a') as usize] += 1;
        }
        for &c in s2.as_bytes() {
            count2[(c - b'a') as usize] += 1;
        }
        if count1 != count2 { return false; }

        let mut sum = 0;
        for i in 0..s1.len() {
            if s1.as_bytes()[i] != s2.as_bytes()[i] {
                sum += 1;
            }
        }

        sum == 2
    }
}