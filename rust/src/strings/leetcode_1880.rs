struct Solution;

impl Solution {
    pub fn convert(b: &[u8]) -> i32 {
        let mut ans = 0_i32;
        for i in 0..b.len() {
            ans = ans * 10 + (b[i] - b'a') as i32;
        }
        ans
    }

    pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        let first = Self::convert(first_word.as_bytes());
        let second = Self::convert(second_word.as_bytes());
        let target = Self::convert(target_word.as_bytes());

        first + second == target
    }
}