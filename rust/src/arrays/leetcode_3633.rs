struct Solution;

impl Solution {
    pub fn earliest_finish_time(ls: Vec<i32>, ld: Vec<i32>, ws: Vec<i32>, wd: Vec<i32>) -> i32 {
        let mut ans = i32::MAX;

        for i in 0..ls.len() {
            for j in 0..ws.len() {
                let a = if ls[i] < ws[j] { (ls[i], ld[i]) } else { (ws[j], wd[j]) };
                let b = if ls[i] < ws[j] { (ws[j], wd[j]) } else { (ls[i], ld[i]) };

                let last_time = 
                    if b.0 >= a.0 + a.1 { b.0 + b.1 } else { a.0 + a.1 + b.1 };
                if last_time < ans { ans = last_time; }
            }
        }

        ans
    }
}