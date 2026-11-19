struct Solution;

impl Solution {
    // 法一：三指针动态规划
    pub fn get_kth_magic_number(k: i32) -> i32 {
        let k = k as usize;
        let mut dp = vec![1i64; k];
        let (mut p3, mut p5, mut p7) = (0, 0, 0);

        for i in 1..k {
            let (next3, next5, next7) = (
                dp[p3] * 3,
                dp[p5] * 5,
                dp[p7] * 7,
            );

            let next = next3.min(next5).min(next7);
            dp[i] = next;

            // 注意：这里必须是三个独立的 if，不能合并成 if-else
            if next == next3 { p3 += 1; }
            if next == next5 { p5 += 1; }
            if next == next7 { p7 += 1; }
        }

        dp[k - 1] as i32
    }
}