struct Solution;

impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        let mut k = 1;
        while k * word.len() < sequence.len() {
            k += 1;
        }

        while k > 0 {
            let tmp = word.as_str().repeat(k);
            if sequence.contains(tmp.as_str()) {
                return k as i32;
            }

            k -= 1;
        }

        k as i32
    }
}