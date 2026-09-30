struct Solution;

impl Solution {
    // 0ms - 击败100%
    pub fn find_middle_index(nums: Vec<i32>) -> i32 {
        // 定义sum[i]表示nums[0]...nums[i - 1]的和
        let mut sum = vec![0; nums.len() + 1];
        let mut acc = 0;
        for i in 0..nums.len() {
            acc += nums[i];
            sum[i + 1] = acc;
        }
        // println!("sum = {:?}", sum);

        for i in 0..nums.len() {
            let left = sum[i];
            let right = sum[sum.len() - 1] - sum[i + 1];
            if left == right { return i as i32; }
            // println!("i = {}, left = {}, right = {}", i, left, right);
        }

        -1
    }
}