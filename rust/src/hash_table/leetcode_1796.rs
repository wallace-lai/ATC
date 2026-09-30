struct Solution;

impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let mut count = [0; 10];
        for c in s.chars() {
            if c.is_digit(10) {
                let num = (c as u8 - b'0') as usize;
                count[num] += 1;
            }
        }

        let n = count.iter().fold(0, |n, i| n + if *i > 0 { 1 } else { 0 });
        if n < 2 { return -1; }

        let mut first = true;
        let mut ans = -1;
        for i in (0..count.len()).rev() {
            if count[i] == 0 { continue; }
            if first { first = false; }
            else {
                ans = i as i32;
                break;
            }
        }

        ans
    }
}