struct Solution;

impl Solution {
    pub fn count_groups(position: Vec<i32>, speed: Vec<i32>, distance: i32) -> i32 {
        let n = position.len();
        let mut stk = Vec::new();

        stk.push(-1);
        for i in 0..n {
            if i == n - 1 || position[i + 1] - position[i] > distance {
                let v = speed[i];
                while stk.len() > 0 && stk[stk.len() - 1] > v {
                    stk.pop();
                }
                stk.push(v);
            }
        }

        stk.len() as i32 - 1
    }
}