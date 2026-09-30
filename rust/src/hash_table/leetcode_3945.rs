struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn digit_frequency_score(mut n: i32) -> i32 {
        let mut count = [0; 10];
        while n > 0 {
            let idx = n % 10;
            count[idx as usize] += 1;
            n /= 10;
        }

        let mut ans = 0;
        for i in 0..count.len() {
            if count[i] > 0 {
                ans += i as i32 * count[i];
            }
        }

        ans
    }
}