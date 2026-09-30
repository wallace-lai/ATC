struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len() as i32;
        let n_square = n * n;
        let sum: i32 = grid.iter().flatten().sum();
        let k = sum - n_square * (n_square + 1) / 2;

        let set1: HashSet<i32> = (1..=n_square).into_iter().collect();
        let set2: HashSet<i32> = grid.into_iter().flatten().collect();
        let b = {
            let mut b = 0;
            for x in set1.difference(&set2) {
                b = *x;
                break;
            }
            b
        };

        vec![b + k, b]
    }
}