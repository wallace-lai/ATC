struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        let mut map1 = HashMap::with_capacity(words1.len());
        let mut map2 = HashMap::with_capacity(words2.len());
        for word in words1 {
            *map1.entry(word).or_insert(0) += 1;
        }
        for word in words2 {
            *map2.entry(word).or_insert(0) += 1;
        }

        let mut ans = 0;
        for (key1, val1) in map1 {
            if let Some(val2) = map2.get(&key1) {
                if val1 == *val2 && val1 == 1 {
                    ans += 1;
                }
            }
        }

        ans
    }
}