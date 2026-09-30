struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans = vec![0, 0];
        let mut left = 0 as i32;
        let mut right = numbers.len() as i32 - 1;

        while left < right {
            let sum = numbers[left as usize] + numbers[right as usize];
            if sum == target {
                ans[0] = left;
                ans[1] = right;
                break;
            } else if sum > target {
                right = right - 1;
            } else {
                left = left + 1;
            }
        }

        ans
    }
}