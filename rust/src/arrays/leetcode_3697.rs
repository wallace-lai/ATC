struct Solution;

impl Solution {
    // 0ms，击败100%
    // 2.15MB，击败100%
    pub fn decimal_representation(mut n: i32) -> Vec<i32> {
        let mut ans = Vec::with_capacity(16);
        let mut base = 1;

        while n > 0 {
            let rem = n % 10;
            if rem != 0 {
                ans.push(rem * base);
            }

            n /= 10;
            base *= 10;
        }

        ans.into_iter().rev().collect()
    }
}