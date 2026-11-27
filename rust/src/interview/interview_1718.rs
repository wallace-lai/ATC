struct Solution;

use std::collections::HashMap;

impl Solution {
    // 法一：滑动窗口
    pub fn shortest_seq(big: Vec<i32>, small: Vec<i32>) -> Vec<i32> {
        // small 为空时，任何窗口都满足
        if small.is_empty() { return vec![]; }

        // 状态一：需求哈希表
        // need[x] > 0 ：还缺 x 这么多个
        // need[x] = 0 ：x 刚好满足
        // need[x] < 0 ：x 在窗口中多了这么多个
        let mut need: HashMap<i32, i32> = HashMap::new();
        for &x in small.iter() {
            *need.entry(x).or_insert(0) += 1;
        }

        // 状态二：还需匹配的元素种类数
        let mut need_count = need.len() as i32;

        // 状态三：结果
        let mut ans_beg: usize = 0;
        let mut ans_len: usize = usize::MAX;
        let mut found = false;

        // 滑动窗口
        let mut left = 0;
        for right in 0..big.len() {
            // 右扩
            let x = big[right];
            if let Some(c) = need.get_mut(&x) {
                if *c > 0 {
                    // 有效填充，短缺种类 -1
                    need_count -= 1;
                }
                *c -= 1;
            }

            // 左缩：当窗口已经覆盖所有目标元素时，尝试收缩左边界找更短子数组
            while need_count == 0 {
                // 记录当前的最优
                let curr_len = right - left + 1;
                if curr_len < ans_len {
                    ans_len = curr_len;
                    ans_beg = left;
                    found = true;
                }

                let y = big[left];
                if let Some(c) = need.get_mut(&y) {
                    *c += 1;
                    if *c > 0 {
                        // 从“恰好”变成“短缺”
                        need_count += 1;
                    }
                }

                left += 1;
            }
        }

        if !found {
            vec![]
        } else {
            let beg = ans_beg as i32;
            let len = ans_len as i32;
            vec![beg, beg + len - 1]
        }
    }
}