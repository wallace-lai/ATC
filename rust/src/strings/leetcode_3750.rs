struct Solution;

impl Solution {
    pub fn minimum_flips(n: i32) -> i32 {
        let n = n as u32;
        let n_rev = n.reverse_bits() >> n.leading_zeros();
        let xor = n ^ n_rev;
        xor.count_ones() as i32
    }
}