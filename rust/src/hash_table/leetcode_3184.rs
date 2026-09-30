struct Solution;

impl Solution {
    // 2ms，击败100%
    pub fn count_complete_day_pairs(hours: Vec<i32>) -> i32 {
        let n = hours.len();
        let mut ans = 0;

        for i in 0..n {
            for j in (i + 1)..n {
                if (hours[i] + hours[j]) % 24 == 0 {
                    ans += 1;
                }
            }
        }

        ans
    }
}