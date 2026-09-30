struct Solution;

impl Solution {
    pub fn count_tested_devices(mut bp: Vec<i32>) -> i32 {
        let mut ans = 0;
        let n = bp.len();

        for i in 0..n {
            if bp[i] > 0 {
                ans += 1;
                for j in (i + 1)..n {
                    bp[j] = std::cmp::max(0, bp[j] - 1);
                }
            }
        }

        ans
    }
}