struct Solution;

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let mut result = letters[0];
        for &c in letters.iter() {
            if c > target {
                result = c;
                break;
            }
        }

        result
    }
}