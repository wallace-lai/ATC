struct Solution;

impl Solution {
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut v1 = Vec::with_capacity(len);
        let mut v2 = Vec::with_capacity(len);

        v1.push(nums[0]);
        v2.push(nums[1]);
        for i in 2..len {
            if v1.last().unwrap() > v2.last().unwrap() {
                v1.push(nums[i]);
            } else {
                v2.push(nums[i]);
            }
        }

        v1.extend(v2);
        v1
    }
}