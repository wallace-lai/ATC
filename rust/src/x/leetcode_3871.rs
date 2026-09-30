struct Solution;

impl Solution {
    const TABLE: [u64; 6] = [
        1_000,
        1_000_000,
        1_000_000_000,
        1_000_000_000_000,
        1_000_000_000_000_000,
        1_000_000_000_000_000_000
    ];

    pub fn count_commas(n: i64) -> i64 {
        let n = n as u64;
        if n < 1000 { return 0; }

        let mut idx = 0;
        while n >= Self::TABLE[idx] {
            idx += 1;
        }

        let mut ans = 0;
        for i in 0..idx {
            if i == idx - 1 {
                ans += (n - Self::TABLE[i]) * (i as u64 + 1);
                continue;
            }

            ans += (Self::TABLE[i + 1] - Self::TABLE[i]) * (i as u64 + 1);
        }

        ans as i64
    }
}