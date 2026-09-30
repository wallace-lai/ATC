struct Solution;

impl Solution {
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        let m = mat.len();
        let n = mat[0].len();
        let k = (k as usize) % n;
        for i in 0..m {
            let mut row = mat[i].clone();
            if i & 1 == 1 {
                row.rotate_right(k);
            } else {
                row.rotate_left(k);
            }
            if row != mat[i] { return false; }
        }
        true
    }
}