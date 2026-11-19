struct Solution;

impl Solution {
    // 法一：Boyer-Moore 投票
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 { return -1; }

        // 找出候选人
        let mut candidate = 0;
        let mut count = 0;

        for &x in nums.iter() {
            if count == 0 {
                candidate = x;
                count = 1;
            } else if x == candidate {
                count += 1;
            } else {
                count -= 1;
            }
        }

        // 验证候选人是否真的超过一半
        let n = nums.iter().filter(|&&x| x == candidate).count();
        if n > nums.len() / 2 { candidate } else { -1 }
    }
}