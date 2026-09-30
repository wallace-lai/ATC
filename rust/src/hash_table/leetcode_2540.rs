struct Solution;

impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let len1 = nums1.len();
        let len2 = nums2.len();
        let mut ans = -1;

        let mut i = 0;
        let mut j = 0;
        while i < len1 && j < len2 {
            let a = &nums1[i];
            let b = &nums2[j];
            if a == b {
                ans = *a;
                break;
            } else if a < b {
                i += 1;
            } else { // a > b
                j += 1;
            }
        }

        ans
    }
}