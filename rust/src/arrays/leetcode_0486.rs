struct Solution;

impl Solution {
    // 法一：错误答案：只看当前，找不到全局最优解
    // pub fn predict_the_winner(nums: Vec<i32>) -> bool {
    //     // println!("nums is {:?}", nums);
    //     let is_odd = (nums.len() & 1) == 1;
    //     let mut sum1 = 0;
    //     let mut sum2 = 0;
    //     let mut left = 0;
    //     let mut right = nums.len() - 1;

    //     while right - left + 1 >= 2 {
    //         if nums[left] > nums[right] {
    //             sum1 += nums[left];
    //             left += 1;
    //         } else {
    //             sum1 += nums[right];
    //             right -= 1;
    //         }

    //         if nums[left] > nums[right] {
    //             sum2 += nums[left];
    //             left += 1;
    //         } else {
    //             sum2 += nums[right];
    //             right -= 1;
    //         }
    //     }

    //     if is_odd { sum1 += nums[left]; }
    //     if sum1 >= sum2 { true } else { false }
    // }

    pub fn predict(nums: &Vec<i32>, beg: i32, end: i32, turn: i32) -> i32 {
        if beg == end {
            return nums[beg as usize] * turn;
        }

        let beg_score = nums[beg as usize] * turn + Self::predict(nums, beg + 1, end, -turn);
        let end_score = nums[end as usize] * turn + Self::predict(nums, beg, end - 1, -turn);
        (beg_score * turn).max(end_score * turn) * turn
    }

    // 法二：O(2^n) - 61ms，击败20%
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        Self::predict(&nums, 0, nums.len() as i32 - 1, 1) >= 0
    }

    // 法三：动态规划 TODO
}