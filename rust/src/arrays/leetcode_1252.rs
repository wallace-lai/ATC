struct Solution;

impl Solution {
    pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let m = m as usize;
        let n = n as usize;
        let len = m * n;
        let mut v = vec![0_u8; len];

        for ind in indices.iter() {
            let row = ind[0] as usize;
            let col = ind[1] as usize;
            for c in 0..n {
                if v[row * n + c] == 1 {
                    v[row * n + c] = 0;
                    ans -= 1;
                } else {
                    v[row * n + c] = 1;
                    ans += 1;
                }
            }
            for r in 0..m {
                if v[r * n + col] == 1 {
                    v[r * n + col] = 0;
                    ans -= 1;
                } else {
                    v[r * n + col] = 1;
                    ans += 1;
                }
            }

        }

        ans
    }
}