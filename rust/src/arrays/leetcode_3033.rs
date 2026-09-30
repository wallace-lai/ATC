struct Solution;

impl Solution {
    pub fn modified_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = matrix.clone();
        let m = ans.len();
        let n = ans[0].len();

        let col_max = |ans: &Vec<Vec<i32>>, c: usize| -> i32 {
            let mut max = ans[0][c];
            for r in 1..m {
                if ans[r][c] > max {
                    max = ans[r][c];
                }
            }
            max
        };

        for r in 0..m {
            for c in 0..n {
                if ans[r][c] == -1 {
                    ans[r][c] = col_max(&ans, c);
                }
            }
        }

        ans
    }
}