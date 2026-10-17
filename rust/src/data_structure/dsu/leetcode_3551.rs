struct Solution;

impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut sum = vec![0; n];

        fn digit_sum(mut x: i32) -> i32 {
            let mut sum = 0;
            while x > 0 {
                sum += x % 10;
                x /= 10;
            }
            sum
        }

        for i in 0..n {
            sum[i] = digit_sum(nums[i]);
        }

        let mut indices: Vec<usize> = (0..n).collect();
        indices.sort_by(|&a, &b| {
            if sum[a] != sum[b] {
                return sum[a].cmp(&sum[b]);
            }
            return nums[a].cmp(&nums[b]);
        });

        let mut tpos = vec![0; n];
        for p in 0..n {
            tpos[indices[p]] = p;
        }

        let mut visited = vec![false; n];
        let mut cycles = 0;
        for i in 0..n {
            if !visited[i] {
                cycles += 1;
                let mut j = i;
                while !visited[j] {
                    visited[j] = true;
                    j = tpos[j];
                }
            }
        }

        n as i32 - cycles
    }
}