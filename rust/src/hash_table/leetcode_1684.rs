struct Solution;

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut count = [0; 26];
        for &c in allowed.as_bytes() {
            let idx = (c - b'a') as usize;
            count[idx] += 1;
        }

        let mut ans = 0;
        for word in words.iter() {
            ans += 1;

            for &c in word.as_bytes() {
                let idx = (c - b'a') as usize;
                if count[idx] == 0 {
                    ans -= 1;
                    break;
                }
            }
        }

        ans
    }
}