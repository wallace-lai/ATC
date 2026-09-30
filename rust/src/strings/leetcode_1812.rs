struct Solution;

impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        let b = coordinates.as_bytes();
        // assert_eq!(b.len(), 2);
        let first = (b[0] - b'a') % 2;
        let second = (b[1] - b'1') % 2;
        first ^ second == 1
    }
}