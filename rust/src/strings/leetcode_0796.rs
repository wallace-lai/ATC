struct Solution;

impl Solution {
    pub fn rotate_string(s: String, g: String) -> bool {
        if s.len() != g.len() { return false; }
        if s == g { return true; }

        let n = s.len();
        for i in 0..(n - 1) {
            let s1 = &s[0..=i];
            let s2 = &s[i + 1..n];
            let g1 = &g[0..n - i - 1];
            let g2 = &g[n - i - 1..n];
            if s1 == g2 && s2 == g1 { return true; }
            // println!("s:[{:?}, {:?}], g[{:?}, {:?}]", s1, s2, g1, g2);
        }

        false
    }
}