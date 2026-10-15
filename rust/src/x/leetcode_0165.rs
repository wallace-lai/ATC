struct Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut v1 = Vec::new();
        let mut v2 = Vec::new();

        fn split(s: &str, v: &mut Vec<i32>) {
            for w in s.split('.')
                .filter(|w| !w.is_empty()) {
                v.push(w.parse::<i32>().unwrap());
            }
        }
        split(&version1, &mut v1);
        split(&version2, &mut v2);

        while v1.len() < v2.len() { v1.push(0); }
        while v2.len() < v1.len() { v2.push(0); }

        if v1 < v2 {
            -1
        } else if v1 > v2 {
            1
        } else {
            0
        }
    }
}