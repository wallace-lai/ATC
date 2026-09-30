struct Solution;

impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        // 由于只能从开头或者末尾拿牌，所以拿完后剩下的必然是一段连续子数组
        // 问题变成了求定长为 n - k 的和最小的连续子数组
        let len = card_points.len() - k as usize;
        let sum: i32 = card_points.iter().sum();
        if len == 0 { return sum; }
        let mut win = 0;
        let mut ans = 0;

        for right in 0..card_points.len() {
            win += card_points[right];
            if right + 1 < len { continue; }

            ans = ans.max(sum - win);
            win -= card_points[right + 1 - len];
        }

        ans
    }
}