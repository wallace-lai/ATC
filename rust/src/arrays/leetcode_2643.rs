struct Solution;

impl Solution {
    pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let m = mat.len();
        let mut ans = vec![0, 0];

        for i in 0..m {
            let ones: i32 = mat[i].iter()
                .filter(|&x| *x == 1)
                .count() as i32;
            if ones > ans[1] {
                ans[0] = i as i32;
                ans[1] = ones;
            }
        }

        ans
    }
}