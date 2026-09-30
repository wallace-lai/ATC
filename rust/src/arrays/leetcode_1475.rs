struct Solution;

impl Solution {
    pub fn final_prices(mut prices: Vec<i32>) -> Vec<i32> {
        let n = prices.len();
        for i in 0..n {
            let mut j = i + 1;
            while j < n && prices[j] > prices[i] {
                j += 1;
            }
            if j < n { prices[i] -= prices[j]; }
        }

        prices
    }
}