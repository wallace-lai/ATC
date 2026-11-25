struct Solution;

impl Solution {
    pub fn pair_sums(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut ans = Vec::new();

        let mut l = 0;
        let mut r = nums.len() as i32 - 1;
        while l < r {
            let (ul, ur) = (l as usize, r as usize);
            let sum = nums[ul] + nums[ur];
    
            if sum == target {
                ans.push(vec![nums[ul], nums[ur]]);
                l += 1;
                r -= 1;
            } else if sum > target {
                r -= 1;
            } else {
                l += 1;
            }
        }

        ans
    }
}