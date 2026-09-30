struct Solution;

impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let mut last_time = vec![0; release_times.len()];
        let mut max_time = release_times[0];
        let mut max_key = keys_pressed.as_bytes()[0];
        last_time[0] = release_times[0];
        for i in 1..release_times.len() {
            last_time[i] = release_times[i] - release_times[i - 1];
            if last_time[i] > max_time {
                max_time = last_time[i];
                max_key = keys_pressed.as_bytes()[i];
            } else if last_time[i] == max_time &&
                keys_pressed.as_bytes()[i] > max_key {
                max_key = keys_pressed.as_bytes()[i];
            }
        }

        max_key as char
    }
}