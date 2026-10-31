struct Solution;

impl Solution {
    pub fn add(a: i32, b: i32) -> i32 {
        let (mut x, mut y) = (a as u32, b as u32);
    
        while y != 0 {
            let carry = (x & y) << 1;
            x ^= y;
            y = carry;
        }

        x as i32
    }
}