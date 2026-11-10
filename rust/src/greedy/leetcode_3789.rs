struct Solution;

impl Solution {
    pub fn minimum_cost(cost1: i32, cost2: i32, cost_both: i32, need1: i32, need2: i32) -> i64 {
        let (cost1, cost2, cost_both, need1, need2) = (
            cost1 as i64, cost2 as i64, cost_both as i64,
            need1 as i64, need2 as i64
        );

        let mut ans = 0;
        let need = std::cmp::min(need1, need2);
        ans += std::cmp::min(cost_both, cost1 + cost2) * need;

        if need1 > need { ans += (need1 - need) * std::cmp::min(cost1, cost_both); }
        if need2 > need { ans += (need2 - need) * std::cmp::min(cost2, cost_both); }

        ans
    }
}