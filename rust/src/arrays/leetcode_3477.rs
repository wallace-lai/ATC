struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, mut baskets: Vec<i32>) -> i32 {
        let mut sum = 0;
        for i in 0..fruits.len() {
            for j in 0..baskets.len() {
                if baskets[j] >= fruits[i] {
                    sum += 1;
                    baskets[j] = -1;
                    break;
                }
            }
        }

        fruits.len() as i32 - sum
    }
}