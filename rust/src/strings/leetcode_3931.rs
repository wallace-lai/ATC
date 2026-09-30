struct Solution;

impl Solution {
    pub fn is_adjacent_diff_at_most_two(s: String) -> bool {
        s.as_bytes().windows(2).all(|w| w[0].abs_diff(w[1]) <= 2)
    }
}