struct Solution;

impl Solution {
    pub fn zigzag_traversal(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let m = grid.len();
        let n = grid[0].len();
        let mut ans = Vec::with_capacity(m * n);

        let mut idx = 0;
        for r in 0..m {
            let row = &grid[r];
            if r & 1 == 0 {
                for c in 0..n {
                    if idx & 1 == 0 {
                        ans.push(row[c]);
                    }
                    idx += 1;
                }
            } else {
                for c in (0..n).rev() {
                    if idx & 1 == 0 {
                        ans.push(row[c]);
                    }
                    idx += 1;
                }
            }
        }   

        ans    
    }
}