struct Solution;

impl Solution {
    /// 用矩阵的第一行和第一列来标记所在行和列是否包含0
    /// 时间复杂度：O(mn)
    /// 空间复杂度：O(1)
    pub fn set_zeroes(mat: &mut Vec<Vec<i32>>) {
        let m = mat.len();
        let n = mat[0].len();
        let mut row0_flag = false;
        let mut col0_flag = false;

        // 第一列是否含有0
        for i in 0..m {
            if mat[i][0] == 0 { col0_flag = true; }
        }

        // 第一行是否含有0
        for j in 0..n {
            if mat[0][j] == 0 { row0_flag = true; }
        }

        // 查找除第一行和第一列之外的所有0
        for i in 1..m {
            for j in 1..n {
                if mat[i][j] == 0 {
                    mat[i][0] = 0;
                    mat[0][j] = 0;
                }
            }
        }

        // 根据第一行和第一列记录的信息进行清零
        for i in 1..m {
            for j in 1..n {
                if mat[i][0] == 0 || mat[0][j] == 0 {
                    mat[i][j] = 0;
                }
            }
        }

        // 最后处理第一行和第一列
        if col0_flag {
            for i in 0..m {
                mat[i][0] = 0;
            }
        }
        if row0_flag {
            for j in 0..n {
                mat[0][j] = 0;
            }
        }
    }
}