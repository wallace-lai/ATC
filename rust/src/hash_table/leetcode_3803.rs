struct Solution;

use std::collections::HashMap;

impl Solution {
    // 0ms，击败100%
    pub fn residue_prefixes(s: String) -> i32 {
        let mut count: HashMap<u8, i32> = HashMap::with_capacity(s.len());
        let mut ans = 0;
        let str = s.as_bytes();

        for i in 0..str.len() {
            *count.entry(str[i]).or_insert(0) += 1;
            let plen = i + 1;
            // 前缀长度模3的值刚好和前缀中不同字符的数量相等
            if plen % 3 == count.len() {
                ans += 1;
            }
        }

        ans
    }
}