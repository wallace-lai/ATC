struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_longest_subarray(array: Vec<String>) -> Vec<String> {
        // 前缀和 --> 该前缀和首次出现的下标
        let mut m: HashMap<i32, i32> = HashMap::new();
        m.insert(0, -1);

        let mut cur_sum = 0;    // 当前的前缀和
        let mut max_len = 0usize;    // 当前最长子数组的长度
        let mut start = 0usize;      // 当前最长子数组的起始位置

        for (i, a) in array.iter().enumerate() {
            // 遇到数字 +1，遇到字母 -1
            if a.as_bytes()[0].is_ascii_digit() {
                cur_sum += 1;
            } else {
                cur_sum += -1;
            }

            if let Some(&prev) = m.get(&cur_sum) {
                // 该前缀和出现过，说明中间这一段的和为0
                let len = (i as i32 - prev) as usize;
                if len > max_len {
                    max_len = len;
                    start = prev as usize + 1;
                }
            } else {
                // 只记录第一次出现的位置，以保证子数组最长
                m.insert(cur_sum, i as i32);
            }
        }

        array[start..start + max_len].to_vec()
    }
}