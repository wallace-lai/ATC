struct Solution;

impl Solution {
    pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
        let &max = nums.iter().max().unwrap();
        let mut num = max;
        let mut ans = 0;
        
        for _ in 0..k {
            ans += num;
            num += 1;
        }

        ans
    }
}