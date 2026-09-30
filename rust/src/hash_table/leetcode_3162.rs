struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
        let n = nums1.len();
        let m = nums2.len();
        let mut ans = 0;

        for i in 0..n {
            for j in 0..m {
                if nums1[i] % (nums2[j] * k) == 0 {
                    ans += 1;
                }
            }
        }

        ans
    }
}