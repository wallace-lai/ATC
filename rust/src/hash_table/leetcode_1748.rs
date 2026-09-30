struct Solution;

impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut count = [0; 128];
        for num in nums {
            count[num as usize] += 1;
        }

        let mut ans = 0;
        for i in 0..count.len() {
            if count[i] == 1 {
                ans += i;
            }
        }

        ans as i32
    }
}