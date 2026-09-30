struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut winner: HashSet<usize> = (0..n).collect();

        for i in 0..n {
            for j in 0..n {
                if i != j && grid[i][j] == 1 {
                    winner.remove(&j);
                }
            }
        }

        let &ans = winner.iter().next().unwrap();
        ans as i32
    }
}