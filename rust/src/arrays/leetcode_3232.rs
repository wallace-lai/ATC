struct Solution;

impl Solution {
    pub fn can_alice_win(nums: Vec<i32>) -> bool {
        let mut sum1 = 0;
        let mut sum2 = 0;
        let mut sum3 = 0;
        for num in nums {
            match num {
                0_i32..=9_i32 => { sum1 += num; },
                10..=99 => { sum2 += num; },
                _ => { sum3 += num; }
            };
        }

        if sum1 > (sum2 + sum3) || sum2 > (sum1 + sum3) {
            return true;
        }

        false
    }
}