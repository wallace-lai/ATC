struct Solution;

impl Solution {
    // 1ms，击败100%
    pub fn build_count(str: &String, count: &mut [i32; 26]) {
        for &c in str.as_bytes() {
            let idx = (c - b'a') as usize;
            count[idx] += 1;
        }
    }

    pub fn min_num_booths(demand: Vec<String>) -> i32 {
        let mut need = [0; 26];
        let mut count = [0; 26];
        for str in demand {
            count.fill(0);
            Self::build_count(&str, &mut count);

            for i in 0..count.len() {
                if count[i] > 0 {
                    need[i] = need[i].max(count[i]);
                }
            }
        }

        need.into_iter().sum()
    }
}