struct Solution;

impl Solution {
    pub fn buy_choco(mut prices: Vec<i32>, money: i32) -> i32 {
        prices.sort_unstable();
        if prices[0] + prices[1] > money {
            return money;
        }

        money - prices[0] - prices[1]
    }
}