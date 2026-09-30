struct Solution;

impl Solution {
    pub fn lucky_numbers(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let m = mat.len();
        let n = mat[0].len();
        let mut ans = Vec::new();

        for c in 0..n {
            // idx为第c列中最大元素所在的行
            let mut idx = 0;
            for r in 0..m {
                if mat[r][c] > mat[idx][c] {
                    idx = r;
                }
            }

            let col_max = mat[idx][c];
            let row = &mat[idx];
            let &row_min = row.iter().min().unwrap();
            if col_max == row_min {
                ans.push(col_max);
            }
        }

        ans
    }
}