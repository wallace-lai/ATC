struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
        let n = matrix.len() as i32;
        let set: HashSet<i32> = (1..=n).into_iter().collect();
        let mut tmp = HashSet::new();

        // 检查所有行
        for i in 0..n {
            tmp.clear();
            let row = &matrix[i as usize];
            for num in row {
                tmp.insert(*num);
            }

            if set != tmp { return false; }
        }

        // 检查所有列
        for j in 0..n {
            tmp.clear();
            for i in 0..n {
                tmp.insert(matrix[i as usize][j as usize]);
            }

            if set != tmp { return false; }
        }

        true
    }
}