struct Solution;

impl Solution {
    pub fn traffic_signal(timer: i32) -> String {
        if timer == 0 {
            "Green".to_string()
        } else if timer == 30 {
            "Orange".to_string()
        } else if 30 < timer && timer <= 90 {
            "Red".to_string()
        } else {
            "Invalid".to_string()
        }
    }
}