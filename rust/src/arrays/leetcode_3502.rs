struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn min_costs(cost: Vec<i32>) -> Vec<i32> {
        let len = cost.len();
        let mut ans = vec![cost[0]; len];
        let mut left_min = cost[0];

        for i in 1..len {
            // 两种办法到达位置i：
            // （1）直接换到位置i，代价为cost[i]
            // （2）先换到位置i的前面代价最小的位置，再免费和位置i互换
            let cost_i = cost[i].min(left_min);
            ans[i] = cost_i;
            if cost_i < left_min { left_min = cost_i; }
        }

        ans
    }
}