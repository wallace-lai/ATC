
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        // num -> [count, i, j]
        let mut m: HashMap<i32, [usize; 3]> = HashMap::with_capacity(nums.len());
        for (i, num) in nums.iter().enumerate() {
            let val = m.entry(*num).or_insert([0, i, i]);
            val[0] += 1;
            val[2] = i;
        }

        let mut d = 0_usize;
        let mut len = 0;
        for (_, val) in &m {
            if val[0] > d {
                d = val[0];
                len = val[2] - val[1] + 1;
            } else if val[0] == d {
                let new_len = val[2] - val[1] + 1;
                len = if new_len < len { new_len } else { len };
            }
        }
        
        len as i32
    }
}