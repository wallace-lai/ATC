struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn max_freq_sum(s: String) -> i32 {
        let mut m = [0; 26];
        for &c in s.as_bytes() {
            let idx = (c - b'a') as usize;
            m[idx] += 1;
        }

        let mut freq1 = i32::MIN;
        let mut freq2 = i32::MIN;
        for i in 0..m.len() {
            let c = i as u8 + b'a';
            if c == b'a' || c == b'e' || c == b'i' || c == b'o' || c == b'u' {
                freq1 = freq1.max(m[i]);
            } else {
                freq2 = freq2.max(m[i]);
            }
        }

        let ans = 
            if freq1 == i32::MIN { 0 } else { freq1 } + 
            if freq2 == i32::MIN { 0 } else { freq2 };

        ans
    }
}