struct Solution;

impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len() as i32;
        let mut ans = 0;

        for x in 0..n {
            for y in 0..n {
                let v = grid[x as usize][y as usize];
                for i in 1..=v {
                    ans = ans +
                    // 上
                    if i == v { 1 } else { 0 } +
                    // 下
                    if i == 1 { 1 } else { 0 } +
                    // 左
                    if y >= 1 && grid[x as usize][y as usize - 1] >= i { 0 } else { 1 } +
                    // 右
                    if y < n - 1 && grid[x as usize][y as usize + 1] >= i { 0 } else { 1 } +
                    // 前
                    if x < n - 1 && grid[x as usize + 1][y as usize] >= i { 0 } else { 1 } +
                    // 后
                    if x >= 1 && grid[x as usize - 1][y as usize] >= i { 0 } else { 1 };
                }
            }
        }

        ans
    }
}