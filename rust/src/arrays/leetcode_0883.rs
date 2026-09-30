struct Solution;

impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let area_xy: i32 = grid.iter()
            .flatten()
            .map(|v| if *v > 0 { 1 } else { 0 })
            .sum();

        let area_yz: i32 = grid.iter()
            .map(|row| row.iter().max().unwrap())
            .sum();

        let n = grid.len();
        let mut area_zx = 0;
        for col in 0..n {
            let mut max = grid[0][col];
            for row in 0..n {
                if grid[row][col] > max {
                    max = grid[row][col];
                }
            }
            area_zx += max;
        }

        area_xy + area_yz + area_zx
    }
}