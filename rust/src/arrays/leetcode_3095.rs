struct Solution;

impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        for d in 1..=n {
            for i in 0..n {
                if i + d > n { break; }
                let mut tmp = 0;
                for k in i..(i + d) {
                    tmp |= nums[k];
                }
                if tmp >= k { return d as i32; }
            }
        }
        
        -1
    }
}
