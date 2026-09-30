struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
        let mut ans = 0;

        let mut map = [0; 128];
        for num in nums {
            map[num as usize] += 1;
        }

        for (i, num) in map.into_iter().enumerate() {
            if num == 2 {
                ans = ans ^ (i as i32);
            }
        }

        ans
    }
}