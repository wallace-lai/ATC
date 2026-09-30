struct Solution;

impl Solution {
    // O(n)，0ms，击败100%
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() < 2 {
            return 0;
        }

        let mut min = i32::MAX;
        let mut max = i32::MIN;

        for &num in nums.iter() {
            if num > max { max = num; }
            if num < min { min = num; }
        }

        let ans;
        if k + min >= max - k {
            ans = 0;
        } else {
            ans = max - min - k - k;
        }
        ans
    }
}