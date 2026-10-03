struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut cnt = 0;
        let mut ans = 0;

        for num in nums {
            if num == 0 {
                cnt = 0;
            } else {
                cnt += 1;
                ans = ans.max(cnt);
            }
        }

        ans
    }
}