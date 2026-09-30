struct Solution;

impl Solution {
    pub fn equal_frequency(word: String) -> bool {
        let mut count = [0; 26];
        for &c in word.as_bytes() {
            let idx = (c - b'a') as usize;
            count[idx] += 1;
        }

        for i in 0..count.len() {
            if count[i] < 1 { continue; }
            count[i] -= 1;
        
            let is_freq_equal = {
                let mut ret = true;
                let mut last_freq = -1;
                for &cnt in count.iter() {
                    if cnt < 1 { continue; }
                    if last_freq < 0 { last_freq = cnt; continue; }
                    if cnt != last_freq {
                        ret = false;
                        break;
                    }
                }
                ret
            };
            if is_freq_equal { return true; }

            count[i] += 1;
        }

        false
    }
}