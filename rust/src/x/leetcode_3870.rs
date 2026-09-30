struct Solution;

impl Solution {
    pub fn count_commas(n: i32) -> i32 {
        std::cmp::max(0, n - 999)
    }
}