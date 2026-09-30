struct Solution;

impl Solution {
    // 0ms，击败100%
    // 3.17M，击败92.59%
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let k = k as usize;
        let t = threshold as usize * k;
        let mut sum = 0;
        let mut ans = 0;

        for right in 0..arr.len() {
            // 扩展窗口
            sum += arr[right];
            if right + 1 < k { continue; }

            // 窗口大小刚好为k
            if sum as usize >= t { ans += 1; }

            // 收缩窗口
            sum -= arr[right + 1 - k];
        }

        ans
    }
}