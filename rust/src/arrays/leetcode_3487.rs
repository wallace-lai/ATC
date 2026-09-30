struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut m = [0; 128];
        for num in nums.iter() {
            if *num <= 0 {
                continue;
            }
            if m[*num as usize] == 0 {
                m[*num as usize] = 1;
            }
        }

        let mut ans = 0;
        for (i, val) in m.into_iter().enumerate() {
            if val == 1 { ans += i as i32; }
        }

        if ans == 0 { ans = nums.into_iter().max().unwrap(); }

        ans
    }
}