struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn max_difference(s: String) -> i32 {
        let mut odd_max = i32::MIN;
        let mut even_min = i32::MAX;
        let mut count = [0; 26];

        for &c in s.as_bytes() {
            let idx = (c - b'a') as usize;
            count[idx] += 1;
        }

        for freq in count {
            if freq < 1 { continue; }
            if freq & 1 == 1 && freq > odd_max { odd_max = freq; }
            if freq & 1 == 0 && freq < even_min { even_min = freq; }
        }

        odd_max - even_min
    }
}