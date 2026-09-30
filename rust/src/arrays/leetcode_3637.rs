struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn is_trionic(nums: Vec<i32>) -> bool {
        let len = nums.len() as i32;
        let mut p = 0;
        let mut q = len - 1;

        while p + 1 < len && nums[p as usize] < nums[p as usize + 1] {
            p += 1;
        }
        if p == 0 || p == len { return false; }

        while q - 1 >= 0 && nums[q as usize - 1] < nums[q as usize] {
            q -= 1;
        }
        if q == len - 1 || q == 0 { return false; }

        if p >= q { return false; }

        for i in p..q {
            if nums[i as usize] < nums[i as usize + 1] { return false; }
        }

        true
    }
}