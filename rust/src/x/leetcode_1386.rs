struct Solution;

use std::collections::HashMap;

impl Solution {
    const TABLE: [u8; 256] = {
        let mut t = [0; 256];
        let mut mask = 0;
        while mask < 256 {
            if (mask & 0xF0) == 0 ||
                (mask & 0x0F) == 0 ||
                (mask & 0x3C) == 0 {
                t[mask] = 1;
            }
            mask += 1;
        }
        t
    };

    // 8ms
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        let mut map = HashMap::new();
        for seat in reserved_seats {
            let col = seat[1] - 1;
            if col < 1 || col > 8 { continue; }
            let row = seat[0] - 1;

            *map.entry(row).or_insert(0_u8) |= 1 << (8 - col);
        }

        let mut ans = 2 * (n - map.len() as i32);
        for &val in map.values() {
            ans += Self::TABLE[val as usize] as i32;
        }

        ans
    }
}