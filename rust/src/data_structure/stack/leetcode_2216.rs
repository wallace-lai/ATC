struct Solution;

impl Solution {
    pub fn min_deletion(nums: Vec<i32>) -> i32 {
        let mut stk = Vec::with_capacity(nums.len());

        let mut m = 0;  // m用于记录发生合并的次数
        for (i, &n) in nums.iter().enumerate() {
            // i 表示n在原数组中的位置
            // i - m 表示发生合并后的数组中的位置
            if stk.len() > 0 && (i - m) % 2 == 1 {
                let top = stk[stk.len() - 1];
                if top == n { // 发生一次合并
                    m += 1;
                    continue;
                }
            }

            stk.push(n);
        }

        // 如果合并后数组长度为奇数，则需要再次删除尾部元素
        if (nums.len() - m) & 1 == 1 { m as i32 + 1 } else { m as i32 }
    }
}
