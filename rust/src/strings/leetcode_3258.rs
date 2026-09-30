struct Solution;

impl Solution {
    // 0ms - O(n) 滑动窗口
    pub fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
        let mut ans = 0;
        let b = s.as_bytes();
        let mut left = 0;
        let mut count = [0, 0];

        for right in 0..b.len() {
            let c = (b[right] - b'0') as usize;
            count[c] += 1;

            while count[0] > k && count[1] > k {
                let l = (b[left] - b'0') as usize;
                count[l] -= 1;
                left += 1;
            }

            ans += right - left + 1;
        }

        ans as i32
    }

    // 0ms - O(n^2) 枚举
    // pub fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
    //     let b = s.as_bytes();
    //     let mut ans = 0;

    //     let mut count = [0, 0];
    //     for i in 0..b.len() {
    //         count.fill(0);
    //         for j in i..b.len() {
    //             let c = (b[j] - b'0') as usize;
    //             count[c] += 1;
    //             if count[0] <= k || count[1] <= k {
    //                 ans += 1;
    //             }
    //             if count[0] > k && count[1] > k {
    //                 break;
    //             }
    //         }
    //     }

    //     ans
    // }
}