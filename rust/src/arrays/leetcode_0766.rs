struct Solution;

impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();

        for i in 1..m {
            for j in 1..n {
                let ni = i - 1;
                let nj = j - 1;
                if matrix[i][j] != matrix[ni][nj] {
                    return false;
                }
            }
        }

        true
    }
}