struct Solution;

impl Solution {
    // 将矩阵先水平翻转再主对角线翻转即可
    pub fn rotate(m: &mut Vec<Vec<i32>>) {
        let n = m.len();
        // 水平翻转
        for x in 0..n / 2 {
            for y in 0..n {
                let tmp = m[x][y];
                m[x][y] = m[n - 1 - x][y];
                m[n - 1 - x][y] = tmp;
            }
        }
        // 沿主对角线翻转
        for x in 0..n {
            for y in 0..x {
                let tmp = m[x][y];
                m[x][y] = m[y][x];
                m[y][x] = tmp;
            }
        }
    }
}