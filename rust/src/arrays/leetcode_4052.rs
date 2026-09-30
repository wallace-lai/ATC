struct Solution;

impl Solution {
    pub fn cyclic_shift(n: i32, mut grid: Vec<Vec<i32>>, row_shift: Vec<i32>, col_shift: Vec<i32>) -> Vec<Vec<i32>> {
        // let mut ans = grid.clone();
        let n = n as usize;

        // row shift
        for r in 0..n {
            let row = &mut grid[r];
            let shift = row_shift[r] as usize % n;
            row.rotate_left(shift);
        }

        // col shift
        let mut ans = grid.clone();
        for c in 0..n {
            let shift = col_shift[c] as usize % n;
            for r in 0..n {
                let nr = (r - shift + n) % n;
                ans[nr][c] = grid[r][c];
            }
        }

        ans
    }
}