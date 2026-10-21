struct Solution;


impl Solution {
    // 法一：哈希表
    // pub fn can_permute_palindrome(s: String) -> bool {
    //     use std::collections::HashMap;

    //     let mut h = HashMap::new();
    //     for &c in s.as_bytes() {
    //         *h.entry(c).or_insert(0) += 1;
    //     }

    //     let count = h.iter().filter(|(_, v)| *v & 1 == 1).count();
    //     count < 2
    // }

    // 法二：更高效版本，位向量压缩
    pub fn can_permute_palindrome(s: String) -> bool {
        let mut bitset = [0u32; 4];
        for &c in s.as_bytes() {
            bitset[(c / 32) as usize] ^= 1u32 << (c % 32) as u32;
        }

        let mut count = 0;
        for i in 0..bitset.len() {
            count += bitset[i].count_ones();
        }
        count < 2
    }
}