struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        let mut map: HashMap<u8, Vec<usize>> = HashMap::with_capacity(s.len());
        for (i, c) in s.as_bytes().iter().enumerate() {
            map.entry(*c).or_insert(vec![]).push(i);
        }

        let mut ans = true;
        for (key, val) in map {
            let idx = (key - b'a') as usize;
            if distance[idx] != (val[1] - val[0] - 1) as i32 {
                ans = false;
                break;
            }
        }

        ans
    }
}