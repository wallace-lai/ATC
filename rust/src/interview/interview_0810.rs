struct Solution;

impl Solution {
    #[cfg(any())]
    // 法一：DFS
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let old_color = image[sr as usize][sc as usize];
        if new_color == old_color { return image; }

        // 从 (x, y) 开始进行染色
        fn dfs(g: &mut Vec<Vec<i32>>, x: i32, y: i32, old_color: i32, new_color: i32) {
            let m = g.len() as i32;
            let n = g[0].len() as i32;

            // 越界
            if x < 0 || x >= m || y < 0 || y >= n {
                return;
            }

            // 遇到非连通块
            let (ux, uy) = (x as usize, y as usize);
            if g[ux][uy] != old_color {
                return;
            }

            g[ux][uy] = new_color;
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                dfs(g, x + dx, y + dy, old_color, new_color);
            }
        }

        dfs(&mut image, sr, sc, old_color, new_color);
        image
    }

    // 法二：显式栈 DFS
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let old_color = image[sr as usize][sc as usize];
        if new_color == old_color { return image; }

        let m = image.len() as i32;
        let n = image[0].len() as i32;
        let mut stk = vec![(sr, sc)];

        while let Some((x, y)) = stk.pop() {
            // 越界
            if x < 0 || x >= m || y < 0 || y >= n {
                continue;
            }

            // 非连通块
            let (ux, uy) = (x as usize, y as usize);
            if image[ux][uy] != old_color {
                continue;
            }

            image[ux][uy] = new_color;
            stk.push((x - 1, y));
            stk.push((x + 1, y));
            stk.push((x, y - 1));
            stk.push((x, y + 1));
        }

        image
    }
}