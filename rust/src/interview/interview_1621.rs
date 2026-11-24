struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn find_swap_values(mut array1: Vec<i32>, mut array2: Vec<i32>) -> Vec<i32> {
        let sum1: i32 = array1.iter().sum();
        let sum2: i32 = array2.iter().sum();
        let (d, exchanged) = if sum1 > sum2 {
            // 保持 array1 是两个数组中和较小的那个
            std::mem::swap(&mut array1, &mut array2);
            (sum1 - sum2, true)
        } else {
            (sum2 - sum1, false)
        };
        if d & 1 == 1 { return vec![]; }
        let halfd = d / 2;

        let s: HashSet<i32> = array2.iter().map(|&i| i).collect();

        // 枚举 x 查找 y
        let mut ans = vec![];
        for &x in array1.iter() {
            let y = x + halfd;
            if s.contains(&y) {
                if exchanged {
                    ans.push(y);
                    ans.push(x);
                } else {
                    ans.push(x);
                    ans.push(y);
                }
                break;
            }
        }

        ans
    }
}