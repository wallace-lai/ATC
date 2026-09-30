struct Solution;

impl Solution {
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = Vec::with_capacity(n);

        for num in nums {
            if num & 1 == 0 {
                ans.push(-1);
                continue;
            }

            let t = num.trailing_ones();
            let x = num - 2_i32.pow(t - 1);
            ans.push(x);
        }

        ans
    }
}