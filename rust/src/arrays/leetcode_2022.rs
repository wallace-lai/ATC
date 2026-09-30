struct Solution;

impl Solution {
    // 0ms
    pub fn construct2_d_array(data: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        if data.len() != (m as usize) * (n as usize) {
            return vec![];
        }

        data.chunks_exact(n as usize)
            .map(|chunk| chunk.to_vec())
            .collect()
    }
}