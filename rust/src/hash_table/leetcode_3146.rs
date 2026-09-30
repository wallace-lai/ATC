struct Solution;

impl Solution {
    pub fn find_permutation_difference(s: String, t: String) -> i32 {
        let mut map = [0; 26];
        let mut str = s.as_bytes();
        for i in 0..str.len() {
            let idx = (str[i] - b'a') as usize;
            map[idx] = i as i32;
        }

        let mut ans = 0;
        str = t.as_bytes();
        for i in 0..str.len() {
            let idx = (str[i] - b'a') as usize;
            ans += map[idx].abs_diff(i as i32);
        }

        ans as i32
    }
}