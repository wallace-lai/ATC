struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn find_degrees(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = Vec::with_capacity(matrix.len());
        for row in matrix.iter() {
            let sum: i32 = row.iter().sum();
            ans.push(sum);
        }

        ans
    }
}