struct Solution;

impl Solution {
    // 法一：基础实现
    pub fn tp1(mat: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = mat.len();
        let m = mat[0].len();
        let mut ret = vec![vec![0; n]; m];

        for i in 0..n {
            for j in 0..m {
                // 读连续但写不连续，破坏缓存局部性
                ret[j][i] = mat[i][j];
            }
        }

        ret
    }

    // 法二：按列填充
    pub fn tp2(mat: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = mat.len();
        let m = mat[0].len();
        let mut ret = vec![vec![0; n]; m];

        for j in 0..m {
            let res_row = &mut ret[j];
            for i in 0..n {
                // 读不连续但写连续
                res_row[i] = mat[i][j];
            }
        }

        ret
    }

    // 法三：分块转置
    pub fn tp3(mat: &Vec<Vec<i32>>, block_size: usize) -> Vec<Vec<i32>> {
        let n = mat.len();
        let m = mat[0].len();
        let mut ret = vec![vec![0; n]; m];

        for i_block in (0..n).step_by(block_size) {
            for j_block in (0..m).step_by(block_size) {
                let i_end = (i_block + block_size).min(n);
                let j_end = (j_block + block_size).min(m);
                // 处理块内元素
                for i in i_block..i_end {
                    for j in j_block..j_end {
                        ret[j][i] = mat[i][j]
                    }
                }
            }
        }

        ret
    }

    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::tp1(&matrix)
    }
}