struct Solution;

impl Solution {
    pub fn min_distinct_freq_pair(nums: Vec<i32>) -> Vec<i32> {
        let mut count = [0; 128];
        for &num in nums.iter() {
            count[num as usize] += 1;
        }

        let mut ans = (-1, -1);

        'outer:
        for x in 0..count.len() {
            if count[x] == 0 { continue; }
            for y in (x + 1)..count.len() {
                if count[y] == 0 { continue; }
                if count[x] != count[y] {
                    ans.0 = x as i32;
                    ans.1 = y as i32;
                    break 'outer;
                }
            }
        }

        vec![ans.0, ans.1]
    }
}