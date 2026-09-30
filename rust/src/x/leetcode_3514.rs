struct Solution;

impl Solution {
    // 法一：时间复杂度 O(n^2 + nm)
    // pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
    //     let m = match nums.iter().max() {
    //         Some(&max) => max as usize,
    //         None => 0 as usize
    //     };
    //     let u = (m + 1).next_power_of_two();

    //     let mut s = vec![0_u16; u];
    //     for i in 0..nums.len() {
    //         for j in 0..nums.len() {
    //             s[(nums[i] ^ nums[j]) as usize] = 1;
    //         }
    //     }

    //     let mut t = vec![0_u16; u];
    //     for i in 0..u {
    //         if s[i] == 0 {
    //             continue;
    //         }

    //         for &num in &nums {
    //             t[(i as i32 ^ num) as usize] = 1;
    //         }
    //     }

    //     let mut ret = 0;
    //     for i in 0..u {
    //         if t[i] > 0 {
    //             ret += 1;
    //         }
    //     }

    //     ret
    // }

    // 法二：动态规划
    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        let m = match nums.iter().max() {
            Some(&max) => max as usize,
            None => 0 as usize
        };
        let u = (m + 1).next_power_of_two();

        let mut one = vec![0_u16; u];
        let mut two = vec![0_u16; u];
        let mut thr = vec![0_u16; u];
        for &num in nums.iter() {
            one[num as usize] = 1;
            for k in 0..u {
                if one[k] > 0 {
                    two[num as usize ^ k] = 1;
                }
            }
        }

        for &num in nums.iter() {
            for k in 0..u {
                if two[k] > 0 {
                    thr[num as usize ^ k] = 1;
                }
            }
        }

        let mut ret = 0;
        for k in 0..u {
            if thr[k] > 0 {
                ret += 1;
            }
        }

        ret
    }
}