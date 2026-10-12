struct Solution;

impl Solution {
    pub fn max_sum(nums: Vec<i32>, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut count = [0; 30];

        for num in nums {
            for i in 0..30 {
                if (num >> i) & 1 == 1 {
                    count[i] += 1;
                }
            }
        }

        let mut ans = 0i64;
        for _ in 0..k {
            let mut curr = 0;
            for i in 0..30 {
                if count[i] > 0 {
                    curr |= 1 << i;
                    count[i] -= 1;
                }
            }
            ans = (ans + curr * curr) % MOD;
        }
    
        ans as i32
    }
}