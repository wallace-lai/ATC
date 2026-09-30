struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn min_absolute_difference(nums: Vec<i32>) -> i32 {
        let mut v1 = Vec::with_capacity(nums.len());
        let mut v2 = Vec::with_capacity(nums.len());

        for (i, num) in nums.iter().enumerate() {
            if *num == 1 { v1.push(i as i32); }
            else if *num == 2 { v2.push(i as i32); }
        }

        let mut ans = u32::MAX;
        for idx1 in v1.iter() {
            for idx2 in v2.iter() {
                let abs = idx1.abs_diff(*idx2);
                if abs < ans { ans = abs; }
            }
        }

        if ans == u32::MAX { -1 } else { ans as i32 }
    }
}