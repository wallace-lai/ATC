struct Solution;

impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort_unstable_by(|a, b| {
            let ua = *a as u32;
            let ub = *b as u32;
            ua.count_ones().cmp(&ub.count_ones())
                .then(ua.cmp(&ub))
        });

        arr
    }
}