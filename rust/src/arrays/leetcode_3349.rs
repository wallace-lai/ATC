struct Solution;

impl Solution {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        let len = k as usize * 2;
        if len > nums.len() { return false; }

        fn is_inc(slice: &[i32]) -> bool {
            let mut ans = true;
            for i in 1..slice.len() {
                if slice[i - 1] >= slice[i] {
                    ans = false;
                    break;
                }
            }
            ans
        }

        for w in nums.windows(len) {
            let w1 = &w[0..w.len() / 2];
            let w2 = &w[w.len() / 2..];
            if is_inc(w1) && is_inc(w2) {
                return true;
            }
        }

        false
    }
}