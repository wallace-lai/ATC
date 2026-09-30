struct Solution;

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        grid.iter()
            .flatten()
            .filter(|&&i| i < 0)
            .count() as i32
    }
}