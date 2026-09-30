struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let m: HashMap<&str, &str> = paths.iter()
            .map(|p| (p[0].as_str(), p[1].as_str()))
            .collect();

        // println!("m is {:?}", m);
        let mut start = paths[0][0].as_str();
        while let Some(end) = m.get(start) {
            start = *end;
        }

        start.to_string()
    }
}