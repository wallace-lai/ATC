struct Solution;

impl Solution {
    pub fn draw_line(length: i32, w: i32, x1: i32, x2: i32, y: i32) -> Vec<i32> {
        let mut ans = vec![0; length as usize];

        let i = ((y * w + x1) / 32) as usize;
        let j = ((y * w + x2) / 32) as usize;

        // 左边界掩码：从 x1%32 到第 31 位为1
        let left_mask: u32 = u32::MAX >> (x1 as u32 % 32);
        // 右边界掩码：从第 0 位到 x2%32 位为1
        let right_mask: u32 = u32::MAX << (31 - x2 as u32 % 32);

        if i == j {
            // 在同一个区间内
            ans[i] = (left_mask & right_mask) as i32;
        } else {
            // 先把 i..=j 全部置成 -1
            for k in i..=j { ans[k] = -1; }
            // 修剪边界
            ans[i] = (ans[i] as u32 & left_mask) as i32;
            ans[j] = (ans[j] as u32 & right_mask) as i32;
        }

        ans
    }
}
