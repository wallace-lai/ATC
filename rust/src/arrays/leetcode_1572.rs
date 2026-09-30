struct Solution;

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let mut ans = 0;
        let mut x = 0;
        let mut y = 0;

        // 主对角线
        while x < n {
            ans += mat[x][y];
            x += 1;
            y += 1;
        }

        // 副对角线
        (x, y) = (0, n - 1);
        while x < n {
            ans += mat[x][y];
            x += 1;
            y -= 1;
        }

        if n & 1 == 1 { ans -= mat[n / 2][n / 2]; }
        ans
    }
}