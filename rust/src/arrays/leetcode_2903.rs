struct Solution;

impl Solution {
    pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
        let index_difference = index_difference as usize;
        let mut ans = vec![-1, -1];
        for i in 0..nums.len() {
            for j in i..nums.len() {
                if j - i >= index_difference &&
                    (nums[i] - nums[j]).abs() >= value_difference {
                        ans[0] = i as i32;
                        ans[1] = j as i32;
                        return ans;
                }
            }
        }
        
        ans
    }
}