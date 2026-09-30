struct Solution;

impl Solution {
    pub fn added_integer(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> i32 {
        nums1.sort_unstable();
        nums2.sort_unstable();
        nums2[0] - nums1[0]
    }
}