struct Solution;

impl Solution {
    pub fn area_of_max_diagonal(d: Vec<Vec<i32>>) -> i32 {
        let mut area = 0;
        let mut diag = 0;

        for i in 0..d.len() {
            let h = d[i][0];
            let w = d[i][1];
            let tmp = h * h + w * w;
            if tmp > diag || (tmp == diag && h * w > area) {
                diag = tmp;
                area = h * w;
            }
        }

        area
    }
}