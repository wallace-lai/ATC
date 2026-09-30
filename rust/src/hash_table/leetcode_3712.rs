struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn sum_divisible_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = [-1; 128];
        for num in nums {
            let idx = num as usize;
            match count[idx] {
                -1 => count[idx] = 1,
                _ => count[idx] += 1,
            }
        }

        let mut ans = 0;
        for i in 0..count.len() {
            if count[i] > -1 && count[i] % k == 0 {
                ans += i as i32 * count[i];
            }
        }

        ans
    }
}