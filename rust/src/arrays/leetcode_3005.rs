struct Solution;

impl Solution {
    // 0ms - 击败100%
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut count = [0_u8; 128];
        for &i in nums.iter() {
            count[i as usize] += 1;
        }

        count.sort_unstable_by(|a, b| b.cmp(a));
        // println!("count is {:?}", count);

        let max = count[0];
        let mut ans = 0;
        for i in 0..count.len() {
            if count[i] == max {
                ans += count[i];
            }
        }

        ans as i32
    }
}