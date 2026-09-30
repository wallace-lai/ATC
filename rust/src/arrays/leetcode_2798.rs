struct Solution;

impl Solution {
    pub fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
         let mut ans = 0;
         for h in hours {
            if h >= target {
                ans += 1;
            }
         }
         ans
    }
}