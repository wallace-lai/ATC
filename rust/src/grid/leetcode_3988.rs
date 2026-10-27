struct Solution;

impl Solution {
    pub fn create_grid(m: i32, n: i32, k: i32) -> Vec<String> {
        let m = m as usize;
        let n = n as usize;
        let mut v = Vec::new();

        for _ in 0..m {
            v.push(vec![b'#'; n]);
        }

        let mut valid = false;

        match k {
            1 => {
                valid = true;
                for j in 0..n {
                    v[0][j] = b'.';
                }
                for i in 1..m {
                    v[i][n - 1] = b'.';
                }
            },
            2 => {
                if m >= 2 && n >= 2 {
                    valid = true;
                    // 2 x 2
                    for j in 0..n - 1 {
                        v[0][j] = b'.';
                    }
                    for i in 0..m {
                        v[i][n - 2] = b'.';
                    }
                    v[m - 2][n - 1] = b'.';
                    v[m - 1][n - 1] = b'.';
                }
            },
            3 => {
                if m >= 3 && n >= 2 {
                    valid = true;
                    // 3 x 2
                    for j in 0..n - 1 {
                        v[0][j] = b'.';
                    }
                    for i in 0..m {
                        v[i][n - 2] = b'.';
                    }
                    v[m - 3][n - 1] = b'.';
                    v[m - 2][n - 1] = b'.';
                    v[m - 1][n - 1] = b'.';
                } else if m >= 2 && n >= 3 {
                    valid = true;
                    // 2 x 3
                    for j in 0..n - 2 {
                        v[0][j] = b'.';
                    }
                    for i in 0..m {
                        v[i][n - 3] = b'.';
                    }
                    v[m - 2][n - 2] = b'.';
                    v[m - 2][n - 1] = b'.';
                    v[m - 1][n - 2] = b'.';
                    v[m - 1][n - 1] = b'.';
                }
            },
            4 => {
                if m >= 4 && n >= 2 {
                    valid = true;
                    // 4 x 2
                    for j in 0..n - 1 {
                        v[0][j] = b'.';
                    }
                    for i in 0..m {
                        v[i][n - 2] = b'.';
                    }
                    v[m - 4][n - 1] = b'.';
                    v[m - 3][n - 1] = b'.';
                    v[m - 2][n - 1] = b'.';
                    v[m - 1][n - 1] = b'.';
                } else if m >= 2 && n >= 4 {
                    valid = true;
                    // 2 x 4
                    for j in 0..n - 3 {
                        v[0][j] = b'.';
                    }
                    for i in 0..m {
                        v[i][n - 4] = b'.';
                    }
                    v[m - 2][n - 3] = b'.';
                    v[m - 2][n - 2] = b'.';
                    v[m - 2][n - 1] = b'.';
                    v[m - 1][n - 3] = b'.';
                    v[m - 1][n - 2] = b'.';
                    v[m - 1][n - 1] = b'.';
                } else if m >= 3 && n >= 3 {
                    valid = true;
                    // 3 x 3
                    for j in 0..n - 2 {
                        v[0][j] = b'.';
                    }
                    for i in 0..m - 3 {
                        v[i][n - 3] = b'.';
                    }
                    v[m - 3][n - 3] = b'.';
                    v[m - 3][n - 2] = b'.';

                    v[m - 2][n - 3] = b'.';
                    v[m - 2][n - 2] = b'.';
                    v[m - 2][n - 1] = b'.';

                    v[m - 1][n - 2] = b'.';
                    v[m - 1][n - 1] = b'.';
                }
            },
            _ => {}
        }

        if !valid { return Vec::new(); }

        let mut ans = Vec::new();
        for row in v {
            ans.push(unsafe {
                String::from_utf8_unchecked(row)
            });
        }

        ans
    }
}