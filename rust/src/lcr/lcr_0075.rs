struct Solution;

use std::collections::HashSet;
use std::collections::HashMap;

impl Solution {
    // 0ms，击败100%
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let s: HashSet<i32> = arr2.iter().cloned().collect();
        let mut m = HashMap::with_capacity(arr1.len());
        let mut v1 = Vec::with_capacity(arr1.len());
        let mut v2 = Vec::with_capacity(arr1.len());

        for num in arr1.into_iter() {
            if !s.contains(&num) {
                v2.push(num);
            } else {
                *m.entry(num).or_insert(0) += 1;
            }
        }

        v2.sort_unstable();
        for num in arr2.into_iter() {
            let mut count = m.get(&num).unwrap().clone();
            while count > 0 {
                v1.push(num);
                count -= 1;
            }
        }
        v1.extend(v2);

        v1
    }
}