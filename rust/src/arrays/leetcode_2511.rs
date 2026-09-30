struct Solution;

impl Solution {
    pub fn capture_forts(forts: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut pre = -1;

        for i in 0..forts.len() {
            if forts[i] == 1 || forts[i] == -1 {
                if pre >= 0 && forts[i] != forts[pre as usize] {
                    ans = ans.max(i as i32 - pre - 1);
                }
                pre = i as i32;
            }
        }

        ans
    }
}