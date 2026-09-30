struct Solution;

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();

        let check_rows = |r: usize, c: usize| {
            let row = &mat[r];
            for i in 0..n {
                if i == c { continue; }
                if row[i] == 1 { return false; }
            }
            true
        };

        let check_cols = |r: usize, c: usize| {
            for i in 0..m {
                if i == r { continue; }
                if mat[i][c] == 1 { return false; }
            }
            true
        };

        let mut ans = 0;
        for r in 0..m {
            for c in 0..n {
                if mat[r][c] == 1 && check_rows(r, c) && check_cols(r, c) {
                    ans += 1;
                }
            }
        }

        ans
    }
}