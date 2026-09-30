struct Solution;

impl Solution {
    pub fn delete_greatest_value(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        for row in grid.iter_mut() {
            row.sort_unstable();
        }

        let mut ans = 0;
        for col in 0..n {
            let mut max = grid[0][col];
            for row in 1..m {
                if grid[row][col] > max {
                    max = grid[row][col];
                }
            }
            ans += max;
        }

        ans
    }
}