struct Solution;

impl Solution {
    pub fn reverse_bits_manual(mut x: u32) -> u32 {
        x = ((x & 0x5555_5555) << 1) | ((x & 0xAAAA_AAAA) >> 1);
        x = ((x & 0x3333_3333) << 2) | ((x & 0xCCCC_CCCC) >> 2);
        x = ((x & 0x0F0F_0F0F) << 4) | ((x & 0xF0F0_F0F0) >> 4);
        x = ((x & 0x00FF_00FF) << 8) | ((x & 0xFF00_FF00) >> 8);
        x = (x  << 16) | (x >> 16);
        x
    }

    pub fn reverse(mut x: u32) -> u32 {
        let mut ans = 0;
        while x > 0 {
            ans = ans * 2 + (x & 1);
            x >>= 1;
        }

        ans
    }

    // 3ms，击败100%
    pub fn sort_by_reflection(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable_by(|&a, &b| {
            let ra = Self::reverse(a as u32);
            let rb = Self::reverse(b as u32);
            ra.cmp(&rb).then(a.cmp(&b))
        });
        nums
    }
}