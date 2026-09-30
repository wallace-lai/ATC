struct Solution;

use std::collections::HashSet;

impl Solution {
    // O(n) - 0ms
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let mut v1 = [0_u8; 128];
        let mut v2 = [0_u8; 128];
        let mut v3 = [0_u8; 128];
        let mut ans = HashSet::with_capacity(128);

        for &val in nums1.iter() {
            v1[val as usize] = 1;
        }
        for &val in nums2.iter() {
            v2[val as usize] = 1;
        }
        for &val in nums3.iter() {
            v3[val as usize] = 1;
        }

        for i in 0..v1.len() {
            if v1[i] > 0 && (v2[i] > 0 || v3[i] > 0) {
                ans.insert(i as i32);
            }
        }
        for i in 0..v2.len() {
            if v2[i] > 0 && (v1[i] > 0 || v3[i] > 0) {
                ans.insert(i as i32);
            }
        }

        ans.into_iter().collect()
    }
}