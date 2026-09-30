struct Solution;

impl Solution {
    pub fn minimum_operations(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let m = grid.len();
        let n = grid[0].len();

        for c in 0..n {
            for r in 1..m {
                if grid[r][c] < grid[r - 1][c] + 1 {
                    ans += grid[r - 1][c] + 1 - grid[r][c];
                    grid[r][c] = grid[r - 1][c] + 1;
                }
            }
        }

        ans
    }
}