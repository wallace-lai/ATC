struct Solution;

impl Solution {
    pub fn min_max_game(mut nums: Vec<i32>) -> i32 {
        let mut new_nums = Vec::with_capacity(nums.len());

        while nums.len() > 1 {
            new_nums.clear();

            if nums.len() == 2 {
                new_nums.push(nums[0].min(nums[1]));
                std::mem::swap(&mut nums, &mut new_nums);
                continue;
            }

            for i in (0..nums.len()).step_by(4) {
                new_nums.push(nums[i].min(nums[i + 1]));
                new_nums.push(nums[i + 2].max(nums[i + 3]));
            }

            std::mem::swap(&mut nums, &mut new_nums);
        }

        nums[0]
    }
}
