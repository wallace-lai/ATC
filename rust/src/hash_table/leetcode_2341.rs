struct Solution;

impl Solution {
    pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len() as i32;

        let mut count = [0_u8; 128];
        for num in nums {
            count[num as usize] += 1;
        }

        let mut remove_count = 0;
        for cnt in count {
            remove_count += cnt as i32 / 2;
        }

        vec![remove_count, len - 2 * remove_count]
    }
}