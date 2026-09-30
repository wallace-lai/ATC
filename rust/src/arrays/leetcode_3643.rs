struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn reverse_submatrix(mut grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
        let mut beg = x as usize;
        let mut end = (x + k - 1) as usize;
        while beg < end {
            for idx in y..(y + k) {
                let idx = idx as usize;
                let t = grid[beg][idx];
                grid[beg][idx] = grid[end][idx];
                grid[end][idx] = t;
            }

            beg += 1;
            end -= 1;
        }

        grid
    }
}