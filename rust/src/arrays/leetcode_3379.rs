struct Solution;

impl Solution {
    pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut v = vec![0; n];

        for i in 0..n {
            if nums[i] > 0 {
                let idx = (i + nums[i] as usize) % n;
                v[i] = nums[idx];
                // println!("i is {i}, idx is {idx}");   
            } else if nums[i] < 0 {
                let k = (-nums[i]) as usize;
                let k = k % n;
                let idx = (i + n + k as usize) % n;
                v[i] = nums[idx];
                // println!("i is {i}, idx is {idx}"); 
            } else {
                v[i] = nums[i];
            }
        }

        v
    }
}