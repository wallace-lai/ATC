struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn gcd(mut a: u32, mut b: u32) -> u32 {
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        a
    }

    // 0ms，击败100%
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut count: HashMap<i32, u32> = HashMap::with_capacity(deck.len());
        for &num in deck.iter() {
            *count.entry(num).or_insert(0) += 1;
        }

        let mut g= 0;
        for (_, &v) in count.iter() {
            g = Self::gcd(g, v);
            if g == 1 {
                return false;
            }
        }

        true
    }
}