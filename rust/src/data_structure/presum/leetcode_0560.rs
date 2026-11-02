struct Solution;

use std::collections::HashMap;

impl Solution {
    // 法一：前缀和 + 哈希表
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as i64;
        let mut presum: HashMap<i64, i32> = HashMap::with_capacity(n + 1);
        // 空前缀和，方便处理以首元素开头到当前节点整段和为 k 的情况
        presum.insert(0, 1);

        let mut curr = 0;
        let mut count = 0;
        for i in 0..n {
            curr += nums[i] as i64;
            // 统计以当前位置为终点，和为 k 的路径条数
            count += *presum.get(&(curr - k)).unwrap_or(&0);
            // 把当前前缀加入哈希表
            *presum.entry(curr).or_insert(0) += 1;
        }

        count
    }
}