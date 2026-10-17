struct Solution;

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let n = num as u32;

        let mut i = 1;
        loop {
            let tmp = i * i;
            if tmp > n { break; }
            if tmp == n { return true; }
            i += 1;
        }

        false
    }
}