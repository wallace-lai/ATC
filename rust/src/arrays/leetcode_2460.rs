struct Solution;

impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        for i in 0..(n - 1) {
            if nums[i] == nums[i + 1] {
                nums[i] *= 2;
                nums[i + 1] = 0;
            }
        }

        let mut zeros = 0;
        let mut ans = Vec::with_capacity(nums.len());
        for &num in nums.iter() {
            if num == 0 {
                zeros += 1;
            } else {
                ans.push(num);
            }
        }
        for _ in 0..zeros {
            ans.push(0);
        }
        
        ans
        // let n = n as i32;
        // let mut left = 0;
        // let mut right = n - 1;
        // while left < right {
        //     while left < n && nums[left as usize] != 0 {
        //         left += 1;
        //     }
        //     while right >= 0 && nums[right as usize] == 0 {
        //         right -= 1;
        //     }

        //     if left < right {
        //         let tmp = nums[left as usize];
        //         nums[left as usize] = nums[right as usize];
        //         nums[right as usize] = tmp;
        //         left += 1;
        //         right -= 1;
        //     }
        // }

        // nums
    }
}