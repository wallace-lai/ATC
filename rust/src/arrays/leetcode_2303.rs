struct Solution;

impl Solution {
    pub fn calculate_tax(b: Vec<Vec<i32>>, mut income: i32) -> f64 {
        let mut ans = 0.0;
        for i in 0..b.len() {
            if income == 0 { break; }
            let d = if i == 0 {
                std::cmp::min(income, b[0][0])
            } else {
                std::cmp::min(income, b[i][0] - b[i - 1][0])
            };
            
            ans += (d * b[i][1]) as f64 / 100.0;
            income -= d;
        }

        ans
    }
}
