struct Solution;

impl Solution {
    pub fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();

        for r in 0..m {
            for c in 0..n {
                if r + 1 < m && grid[r][c] != grid[r + 1][c] {
                    return false;
                }
                if c + 1 < n && grid[r][c] == grid[r][c + 1] {
                    return false;
                }
            }
        }

        true
    }
}