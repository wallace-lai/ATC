struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let set1: HashSet<i32> = nums1.iter().copied().collect();
        let set2: HashSet<i32> = nums2.iter().copied().collect();

        let mut cnt1 = 0;
        for num in nums1 {
            if set2.contains(&num) { cnt1 += 1; }
        }

        let mut cnt2 = 0;
        for num in nums2 {
            if set1.contains(&num) { cnt2 += 1; }
        }

        vec![cnt1, cnt2]
    }
}