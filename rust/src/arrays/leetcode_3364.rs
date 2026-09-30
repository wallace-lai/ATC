struct Solution;

impl Solution {
    pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        let mut ans = i32::MAX;
        let l = l as usize;
        let r = r as usize;
        let n = nums.len();
        // pre[i] = sum(nums[0..=i])
        let mut pre = vec![0; n];

        for i in 0..n {
            if i == 0 {
                pre[i] = nums[i];
                continue;
            }
            pre[i] = nums[i] + pre[i - 1];
        }

        for d in l..=r {
            for i in 0..n {
                if i + d > n { break; }
                // nums[i..(i + d)]
                let sum = pre[i + d - 1] - if i == 0 { 0 } else { pre[i - 1] };
                if sum > 0 && sum < ans { ans = sum; }
            }
        }

        if ans == i32::MAX { -1 } else { ans }
    }
}