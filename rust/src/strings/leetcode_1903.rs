struct Solution;

impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        let pos = num.rfind(|c| {
            c == '1' || c == '3' || c == '5' ||
            c == '7' || c == '9'
        });

        if let Some(idx) = pos {
            let slice = &num.as_str()[0..=idx];
            return slice.to_string();
        }

        "".to_string()
    }
}