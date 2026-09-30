struct Solution;

impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        let count = s.chars()
            .filter(|c| *c == letter)
            .count();

        let percent = count as f32 / s.len() as f32 * 100.0;
        percent.round() as i32
    }
}