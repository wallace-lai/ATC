struct Solution;

impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let len = bits.len();
        let mut i = 0;
        while i < len.checked_sub(1).unwrap_or(0) {
            i += (bits[i] as usize) + 1;
        }

        if i == len - 1 { true } else { false }
    }
}