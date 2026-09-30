struct Solution;

impl Solution {
    pub fn find_rotation(mut mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        if mat == target { return true; }
        let n = mat.len();
        let mut tmp = vec![vec![0; n]; n];

        // 90
        for r in 0..n {
            for c in 0..n {
                tmp[c][n - 1 - r] = mat[r][c];
            }
        }
        if tmp == target { return true; }
        std::mem::swap(&mut tmp, &mut mat);

        // 180
        for r in 0..n {
            for c in 0..n {
                tmp[c][n - 1 - r] = mat[r][c];
            }
        }
        if tmp == target { return true; }
        std::mem::swap(&mut tmp, &mut mat);

        // 270
        for r in 0..n {
            for c in 0..n {
                tmp[c][n - 1 - r] = mat[r][c];
            }
        }
        if tmp == target { return true; }

        false
    }
}