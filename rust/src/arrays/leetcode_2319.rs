struct Solution;

impl Solution {
    pub fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool {
        let n = grid.len();
        let mut non_zero = 0;
        let (mut x, mut y) = (0, 0);
        while x < n {
            if grid[x][y] == 0 { return false; }
            non_zero += 1;
            x += 1;
            y += 1;
        }

        (x, y) = (0, n - 1);
        while x < n {
            if grid[x][y] == 0 { return false; }
            non_zero += 1;
            x += 1;
            y -= 1;
        }

        if n & 1 == 1 { non_zero -= 1; }

        let zero_num = grid.iter()
            .flatten()
            .filter(|&i| *i == 0)
            .count();

        if non_zero + zero_num == n * n {
            true
        } else {
            false
        }
    }
}