struct Solution;

impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        if n < 2 { return vec![0]; }
        let mut pre = vec![0; n];
        let mut suf = vec![0; n];

        for i in 1..n {
            pre[i] = nums[i - 1] + pre[i - 1];
        }
        for i in (0..=(n - 2)).rev() {
            suf[i] = nums[i + 1] + suf[i + 1];
        }
        // println!("pre is {:?}", pre);
        // println!("suf is {:?}", suf);

        let mut ans = vec![0; n];
        for i in 0..n {
            ans[i] = (pre[i] - suf[i]).abs();
        }

        ans
    }
}