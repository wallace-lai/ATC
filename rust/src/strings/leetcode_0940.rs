struct Solution;

impl Solution {
    const MOD: i32 = 1000000007;

    pub fn distinct_subseq_ii(s: String) -> i32 {
        let mut last = [-1_i32; 26];

        let n = s.len();
        let mut dp = vec![1; n];
        for i in 0..n {
            for j in 0..26 {
                if last[j] != -1 {
                    dp[i] = (dp[i] + dp[last[j] as usize]) % Self::MOD;
                }
            }

            let idx = (s.as_bytes()[i] - b'a') as usize;
            last[idx] = i as i32;
        }

        let mut ans = 0;
        for i in 0..26 {
            if last[i] != -1 {
                ans = (ans + dp[last[i] as usize]) % Self::MOD;
            }
        }

        ans
    }
}