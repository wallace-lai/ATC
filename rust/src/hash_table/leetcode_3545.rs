struct Solution;

impl Solution {
    pub fn min_deletion(s: String, k: i32) -> i32 {
        let mut m = [0; 26];
        for &c in s.as_bytes() {
            let idx = (c - b'a') as usize;
            m[idx] += 1;
        }

        let mut v: Vec<i32> = m.into_iter().filter(|x| *x > 0).collect();
        if v.len() <= k as usize { return 0; }
        v.sort_unstable();


        let mut ans = 0;
        for i in 0..(v.len() - k as usize) {
            ans += v[i];
        }

        ans
    }
}