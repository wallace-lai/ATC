struct Solution;

impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        let nn = n - 2;

        let mut ans = vec![vec![0; nn]; nn];
        for i in 0..=(n - 3) {
            for j in 0..=(n - 3) {
                let mut max = grid[i][j];
                for x in i..(i + 3) {
                    for y in j..(j + 3) {
                        if grid[x][y] > max {
                            max = grid[x][y];
                        }
                    }
                }
                ans[i][j] = max;
            }
        }

        ans
    }
}