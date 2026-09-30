struct Solution;

use std::collections::HashSet;

impl Solution {
    // 0ms，击败100%
    pub fn check_prime_frequency(nums: Vec<i32>) -> bool {
        let v = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29,
            31, 37, 41, 43, 47, 53, 59, 61, 67,
            71, 73, 79, 83, 89, 97, 101
        ];
        let s: HashSet<i32> = v.into_iter().collect();
        let mut m = [0; 128];
        for num in nums {
            m[num as usize] += 1;
        }

        let mut ans = false;
        for i in 0..m.len() {
            if m[i] > 0 && s.contains(&m[i]) {
                ans = true;
                break;
            }
        }

        ans
    }
}