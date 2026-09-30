struct Solution;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let sn = s.len();
        let tn = t.len();
        if sn < tn { return 0; }

        let mut dp = vec![vec![0; tn + 1]; sn + 1];
        for i in 0..=sn {
            dp[i][tn] = 1;
        }

        for i in (0..sn).rev() {
            let schar = s.as_bytes()[i];
            for j in (0..tn).rev() {
                let tchar = t.as_bytes()[j];
                if schar == tchar {
                    dp[i][j] = dp[i + 1][j + 1] + dp[i + 1][j];
                } else {
                    dp[i][j] = dp[i + 1][j];
                }
            }
        }
        
        dp[0][0]
    }
}