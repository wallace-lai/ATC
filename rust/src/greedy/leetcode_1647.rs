struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let mut count = [0; 26];
        for &c in s.as_bytes() {
            let idx = (c - b'a') as usize;
            count[idx] += 1;
        }

        let mut ans = 0;
        count.sort_unstable_by(|a, b| b.cmp(a));
        let mut set = HashSet::with_capacity(26);

        for i in 0..26 {
            if count[i] == 0 { break; }
            while set.contains(&count[i]) {
                ans += 1;
                count[i] -= 1;
            }
            if count[i] > 0 { set.insert(count[i]); }
        }

        ans
    }
}
