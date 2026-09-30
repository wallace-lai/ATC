struct Solution;

impl Solution {
    pub fn rearrange_characters(s: String, target: String) -> i32 {
        let mut tcnt = [0; 26];
        let mut scnt = [0; 26];

        for &c in target.as_bytes() {
            let idx = (c - b'a') as usize;
            tcnt[idx] += 1;
        }
        for &c in s.as_bytes() {
            let idx = (c - b'a') as usize;
            scnt[idx] += 1;
        }

        let mut prod = i32::MAX;
        for i in 0..tcnt.len() {
            if tcnt[i] > 0 {
                prod = prod.min(scnt[i] / tcnt[i]);
                if prod == 0 {
                    return 0;
                }
            }
        }
        
        prod
    }
}