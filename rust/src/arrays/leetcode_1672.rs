struct Solution;

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let max: i32 = accounts.iter()
            .map(|acc| {
                acc.iter().sum()
            })
            .max()
            .unwrap();

        max
    }
}