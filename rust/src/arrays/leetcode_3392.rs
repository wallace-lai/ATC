struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for w in nums.windows(3) {
            if w[1] & 1 != 0 { continue; }
            if w[0] + w[2] == w[1] / 2 {
                ans += 1;
            }
        }
        ans
    }
}