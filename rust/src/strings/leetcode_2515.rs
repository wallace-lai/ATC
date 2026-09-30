struct Solution;

impl Solution {
    pub fn closest_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let n = words.len();
        let i = start_index as usize;
        let mut d = 0;
        while d < n {
            let prev = (i - d + n) % n;
            let next = (i + d) % n;
            if words[prev] == target || words[next] == target {
                return d as i32;
            }

            d += 1;
        }

        -1
    }
}