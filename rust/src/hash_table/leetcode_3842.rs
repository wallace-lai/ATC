struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn toggle_light_bulbs(bulbs: Vec<i32>) -> Vec<i32> {
        let mut count = [0; 128];
        for b in bulbs {
            count[b as usize] += 1;
        }

        let mut ans = vec![];
        for i in 0..count.len() {
            if count[i] > 0 && count[i] & 1 == 1 {
                ans.push(i as i32);
            }
        }

        ans
    }
}