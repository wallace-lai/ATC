struct Solution;

impl Solution {
    pub fn longest_alternating_subarray(nums: Vec<i32>, threshold: i32) -> i32 {
        // 1. 以偶数开头
        // 2. 偶数奇数交替
        // 3. 值小于等于threshold
        let n = nums.len();
        let mut ans = 0;
        let mut start = None;
        for i in 0..n {
            if start.is_none() {
                if nums[i] <= threshold && nums[i] & 1 == 0 {
                    start = Some(i);
                }
            } else {
                if nums[i] > threshold ||
                    nums[i - 1] % 2 == nums[i] % 2 {
                    let len = i - start.unwrap();
                    if len > ans { ans = len; }
                    start = None;

                    if nums[i] <= threshold && nums[i] & 1 == 0 {
                        start = Some(i);
                    }
                }
            }
        }
        if start.is_some() {
            let len = n - start.unwrap();
            if len > ans { ans = len; }
        }

        ans as i32
    }
}