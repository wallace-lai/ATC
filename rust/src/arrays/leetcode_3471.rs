struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = -1;
        let len = nums.len();

        if k == 1 {
            let mut count = [0; 64];
            for num in nums { count[num as usize] += 1; }
            for (i, val) in count.iter().enumerate().rev() {
                if *val == 1 { ans = i as i32; break; }
            }
        } else if k == len as i32 {
            let max = nums.iter().max().unwrap();
            ans = *max; 
        } else {
            let mut tmp = i32::MIN;
            let mut count = [0; 64];
            for num in &nums { count[*num as usize] += 1; }
            if count[nums[0] as usize] == 1 { tmp = tmp.max(nums[0]); }
            if count[nums[len - 1] as usize] == 1 { tmp = tmp.max(nums[len - 1]); }
            if tmp != i32::MIN { ans = tmp; }
        }

        ans
    }
}