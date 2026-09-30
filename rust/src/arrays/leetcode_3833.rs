struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn dominant_indices(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 1 { return 0; }

        let mut ans = 0;
        let mut suf = vec![nums[len - 1]; len];
        for i in (0..(nums.len() - 1)).rev() {
            suf[i] = nums[i] + suf[i + 1];
            let avg = suf[i + 1] as f32 / (len - i - 1) as f32;
            if nums[i] as f32 > avg { ans += 1; }
        }

        ans
    }
}