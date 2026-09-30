struct Solution;

impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let mut line = 1;
        let mut width = 0;
        for &c in s.as_bytes() {
            let need = widths[(c - b'a') as usize];
            width += need;
            if width > 100 {
                line += 1;
                width = need;
            }
        }

        vec![line, width]
    }
}