struct Solution;

impl Solution {
    pub fn ways_to_step(n: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        // 题目保证 n >= 1
        match n {
            1 => 1,
            2 => 2,
            3 => 4,
            _ => {
                let mut f1 = 1;
                let mut f2 = 2;
                let mut f3 = 4;
                let mut f4 = 0;
                for _ in 4..=n {
                    f4 = ((f1 + f2) % MOD + f3) % MOD;
                    f1 = f2; f2 = f3; f3 = f4;
                }
                f4
            }
        }
    }
}