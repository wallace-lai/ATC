struct Solution;

impl Solution {
    #[cfg(any())]
    // 法一：二分搜索（往左）
    // 思路：当二分到的 mid 是空串时，必须向左或者向右线性找到一个非空串 p
    // 用 words[p] 与 target 比较来决定舍弃哪个半区
    pub fn find_string(words: Vec<String>, s: String) -> i32 {
        if s.is_empty() { return -1; }

        let mut l = 0;
        let mut r = words.len() as i32 - 1;
        while l <= r {
            let m = l + (r - l) / 2;

            // 当 mid 为空时，向左找最近的非空字符串
            let mut p = m;
            while p >= l && words[p as usize].is_empty() {
                p -= 1;
            }

            // 当左侧 [l, m] 全是空串，目标在右侧
            if p < l {
                l = m + 1;
                continue;
            }

            if words[p as usize] == s {
                return p;
            }

            if words[p as usize] < s {
                // p 到 m 之间全是空串，可以跳过
                l = m + 1;
            } else {
                r = p - 1;
            }
        }

        -1
    }

    // 法二：二分查找（往右）
    pub fn find_string(words: Vec<String>, s: String) -> i32 {
        if s.is_empty() { return -1; }

        let mut l = 0;
        let mut r = words.len() as i32 - 1;
        while l <= r {
            let m = l + (r - l) / 2;
            
            // 当 mid 为空时，向右找最近的非空字符串
            let mut p = m;
            while p <= r && words[p as usize].is_empty() {
                p += 1;
            }

            // 当右侧 [m, r] 全是空串时，目标在左侧
            if p > r {
                r = m - 1;
                continue;
            }

            if words[p as usize] == s {
                return p;
            }

            if words[p as usize] < s {
                // m 到 p 之间全是空串，可跳过
                l = p + 1;
            } else {
                r = m - 1;
            }
        }

        -1
    }
}