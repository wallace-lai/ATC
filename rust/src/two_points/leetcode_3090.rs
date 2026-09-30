struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn maximum_length_substring(s: String) -> i32 {
        let mut ans = 0;
        let mut window = [0; 26];
        let mut left = 0;

        for right in 0..s.len() {
            let push = s.as_bytes()[right];
            let mut push_count = {
                let idx = (push - b'a') as usize;
                window[idx] += 1;
                window[idx]
            };

            while push_count > 2 {
                let pop = s.as_bytes()[left];
                let idx = (pop - b'a') as usize;
                assert!(window[idx] >= 1);
                window[idx] -= 1;
                if pop == push { push_count -= 1; }

                left += 1;
            }

            ans = ans.max(right - left + 1);
        }

        ans as i32
    }
}