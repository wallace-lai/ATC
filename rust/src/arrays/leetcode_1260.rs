
struct Solution;

impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let cols = grid[0].len();
        let mut v: Vec<i32> = grid.into_iter().flatten().collect();

        let nk = (k as usize) % v.len();
        v.rotate_right(nk);
        v.chunks_exact(cols)
            .map(|chunk| chunk.to_vec())
            .collect()
    }
}