struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = [0; 128];
        for num in nums {
            if num < k { return -1; }
            if num > k { count[num as usize] += 1; }
        }

        let mut ans = 0;
        for num in count {
            if num > 0 { ans += 1; }
        }

        ans
    }
}