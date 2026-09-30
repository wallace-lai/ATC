struct Solution;

impl Solution {
    pub fn sum(mut n: i32) -> i32 {
        let mut ans = 0;
        while n > 0 {
            ans += n % 10;
            n /= 10;
        }

        ans
    }

    // 0ms，击败100%
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
        let mut ans = -1;
        for (i, num) in nums.into_iter().enumerate() {
            if i as i32 == Self::sum(num) {
                ans = i as i32;
                break;
            }
        }

        ans
    }
}