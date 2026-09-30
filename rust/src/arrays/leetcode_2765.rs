struct Solution;

impl Solution {
    pub fn alternating_subarray(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        nums.push(-1);
        let mut ans = -1;

        for i in 0..(n - 1) {
            if nums[i] + 1 == nums[i + 1] {
                for j in (i + 2)..=n {
                    if nums[j] != nums[j - 2] {
                        let len = (j - i) as i32;
                        if len > ans { ans = len; }
                        break;
                    }
                }
            }
        }
        
        ans as i32
    }
}
