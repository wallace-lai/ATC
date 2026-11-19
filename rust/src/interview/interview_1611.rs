struct Solution;

impl Solution {
    pub fn diving_board(shorter: i32, longer: i32, k: i32) -> Vec<i32> {
        if k == 0 { return vec![]; }
        if shorter == longer { return vec![k * shorter]; }

        let mut ans = Vec::new();
        let d = longer - shorter;
        for i in 0..=k {
            ans.push(shorter * k + d * i);
        }
        ans
    }
}