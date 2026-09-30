struct Solution;

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut v: Vec<(String, i32)> = names.into_iter().zip(heights.into_iter()).collect();
        v.sort_by(|a, b| { b.1.cmp(&a.1) });
        v.into_iter().map(|pair| { pair.0 }).collect()
    }
}