struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // value --> weight
        let mut map = HashMap::new();
        for item in items1 {
            *map.entry(item[0]).or_insert(0) += item[1];
        }
        for item in items2 {
            *map.entry(item[0]).or_insert(0) += item[1];
        }

        let mut v: Vec<Vec<i32>> = map.into_iter()
            .map(|(value, weight)| vec![value, weight])
            .collect();
        v.sort_unstable_by(|a, b| a[0].cmp(&b[0]));

        // println!("v is {:?}", v);

        v
    }
}