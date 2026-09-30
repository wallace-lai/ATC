struct Solution;

impl Solution {
    pub fn build_count(s: &String, count: &mut [i32; 26]) {
        for &c in s.as_bytes() {
            let idx = (c - b'a') as usize;
            count[idx] += 1;
        }
    }

    pub fn is_good(wcount: &[i32; 26], ccount: &[i32; 26]) -> bool {
        for i in 0..wcount.len() {
            if wcount[i] > 0 && wcount[i] > ccount[i] {
                return false;
            }
        }

        true
    }

    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut wcount = [0; 26];
        let mut ccount = [0; 26];
        Self::build_count(&chars, &mut ccount);

        let mut ans = 0;
        for w in words.iter() {
            wcount.fill(0);
            Self::build_count(w, &mut wcount);
            if Self::is_good(&wcount, &ccount) {
                ans += w.len();
            }
        }
        
        ans as i32
    }
}