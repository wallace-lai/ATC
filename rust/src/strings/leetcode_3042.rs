struct Solution;

impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        // 检查s1是否同时是s2的前缀和后缀
        fn is_prefix_and_suffix(s1: &str, s2: &str) -> bool {
            if s1.len() > s2.len() { return false; }
            let n = s1.len();
            
            for i in 0..n {
                if s1.as_bytes()[i] != s2.as_bytes()[i] {
                    return false;
                }
            }
            let d = s2.len() - n;
            for i in (0..n).rev() {
                if s1.as_bytes()[i] != s2.as_bytes()[i + d] {
                    return false;
                }
            }

            true
        }

        let mut ans = 0;
        for i in 0..words.len() {
            for j in (i + 1)..words.len() {
                if is_prefix_and_suffix(&words[i], &words[j]) {
                    ans += 1;
                }
            }
        }

        ans
    }
}