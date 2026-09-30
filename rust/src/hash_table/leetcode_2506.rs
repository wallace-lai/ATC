struct Solution;

impl Solution {
    pub fn similar_pairs(words: Vec<String>) -> i32 {
        let mut count: Vec<[u8; 26]> = Vec::with_capacity(words.len());
        for i in 0..words.len() {
            let mut cnt = [0; 26];
            for &c in words[i].as_bytes() {
                let idx = (c - b'a') as usize;
                cnt[idx] = 1;
            }
            count.push(cnt);
        }

        let mut ans = 0;
        for i in 0..words.len() {
            for j in (i + 1)..words.len() {
                if count[i] == count[j] {
                    ans += 1;
                }
            }
        }
        ans
    }
}