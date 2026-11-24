struct Solution;

use std::collections::HashSet;

impl Solution {
    // 法一：模拟
    pub fn print_k_moves(k: i32) -> Vec<String> {
        const DIR: [[i32; 2]; 4] = [
            // 方向：右下左上
            [0, 1], [1, 0], [0, -1], [-1, 0]
        ];

        let i2b = |i: usize| {
            match i {
                0 => b'R',
                1 => b'D',
                2 => b'L',
                3 => b'U',
                _ => unreachable!()
            }
        };

        // 维护动态边界
        let mut minx = 0;
        let mut miny = 0;
        let mut maxx = 0;
        let mut maxy = 0;

        // 保存黑色格子的坐标
        let mut black: HashSet<(i32, i32)> = HashSet::new();

        // 当前的位置和朝向，初始位置 (0, 0)，方向 0（右）
        let mut curr = ((0, 0), 0usize);

        for _ in 0..k {
            if black.contains(&curr.0) {
                // 如果当前格子是黑色
                black.remove(&curr.0);

                let i = (curr.1 + 3) % 4;
                curr.1 = i;
                curr.0.0 += DIR[i][0];
                curr.0.1 += DIR[i][1];

            } else {
                // 如果当前格子是白色
                black.insert(curr.0);

                let i = (curr.1 + 1) % 4;
                curr.1 = i;
                curr.0.0 += DIR[i][0];
                curr.0.1 += DIR[i][1];
            }

            minx = minx.min(curr.0.0);
            miny = miny.min(curr.0.1);
            maxx = maxx.max(curr.0.0);
            maxy = maxy.max(curr.0.1);
        }

        let m = maxx - minx + 1;
        let n = maxy - miny + 1;
        let mut ans = Vec::with_capacity(m as usize);

        for _ in 0..m {
            let mut row = String::with_capacity(n as usize);
            row.extend(std::iter::repeat('_').take(n as usize));
            ans.push(row);
        }

        // 填充黑色格子
        for (ix, iy) in black.iter() {
            let x = (ix - minx) as usize;
            let y = (iy - miny) as usize;
            unsafe { ans[x].as_bytes_mut()[y] = b'X'; }
        }

        // 填充蚂蚁所在网格
        let x = (curr.0.0 - minx) as usize;
        let y = (curr.0.1 - miny) as usize;
        unsafe { ans[x].as_bytes_mut()[y] = i2b(curr.1); }
        
        ans
    }
}