struct Solution;

impl Solution {
    // 超时
    // pub fn max_vowels(s: String, k: i32) -> i32 {
    //     let k = k as usize;
    //     let mut ans = 0;
    //     for w in s.as_bytes().windows(k) {
    //         let count = w.iter().filter(|&&c| "aeiou".contains(c as char)).count();
    //         ans = ans.max(count);
    //     }
    //     ans as i32
    // }

    pub fn max_vowels(s: String, k: i32) -> i32 {
        let k = k as usize;
        let b = s.as_bytes();
        const VOWELS: &str = "aeiou";

        let mut cnt = 0;
        let mut ans = 0;
        for right in 0..b.len() {
            // 扩展窗口
            if VOWELS.contains(b[right] as char) {
                cnt += 1;
            }
            if right + 1 < k { continue; }

            // 窗口大小刚好为k
            ans = ans.max(cnt);

            // 收缩窗口
            if VOWELS.contains(b[right + 1 - k] as char) {
                cnt -= 1;
            }
        }

        ans
    }
}