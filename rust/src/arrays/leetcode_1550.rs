struct Solution;

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        arr.windows(3)
            .any(|w| {
                w[0] & 1 == 1 &&
                w[1] & 1 == 1 &&
                w[2] & 1 == 1
            })
    }
}