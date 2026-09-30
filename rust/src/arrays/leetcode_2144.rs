struct Solution;

impl Solution {
    pub fn minimum_cost(mut cost: Vec<i32>) -> i32 {
        cost.sort_unstable();
        let n = cost.len();
        if n <= 2 { return cost.iter().sum(); }

        let mut ans = 0;
        while cost.len() >= 2 {
            let a = cost.pop().unwrap();
            let b = cost.pop().unwrap();
            ans += a + b;

            if let Some(&val) = cost.last() {
                if val <= std::cmp::min(a, b) {
                    cost.pop();
                }
            }
        }
        let remain: i32 = cost.iter().sum();
        ans += remain;

        ans
    }
}