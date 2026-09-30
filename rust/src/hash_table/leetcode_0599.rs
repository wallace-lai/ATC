struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let index_map: HashMap<String, usize> = list1
            .into_iter()
            .enumerate()
            .map(|(i, name)| (name, i))
            .collect();

        let mut ans = Vec::with_capacity(list2.len());
        let mut sum = usize::MAX;

        for (j, name) in list2
            .into_iter()
            .enumerate() {
            if let Some(&i) = index_map.get(&name) {
                let curr_sum = i + j;
                if curr_sum < sum {
                    sum = curr_sum;
                    ans.clear();
                    ans.push(name);
                } else if curr_sum == sum {
                    ans.push(name);
                }
            }
        }

        ans
    }
}

