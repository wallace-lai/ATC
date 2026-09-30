struct Solution;

impl Solution {
    pub fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool {
        let m = m as usize;
        let k = k as usize;
        for w in arr.windows(m * k) {
            let mut tmp = true;
            for i in 1..k {
                let prev = &w[(i - 1) * m..i * m];
                let curr = &w[i * m..(i + 1) * m];
                if prev != curr {
                    tmp = false;
                    break;
                }
            }
            if tmp { return true; }
        }

        false
    }
}