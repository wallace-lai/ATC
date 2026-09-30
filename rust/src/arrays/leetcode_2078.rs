struct Solution;

impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let n = colors.len() as i32;
        let mut ans = 0;

        for i in 0..n {
            let mut tmp = 0;

            let mut left = i - 1;
            while left >= 0 {
                if colors[i as usize] != colors[left as usize] {
                    tmp = tmp.max(i - left);
                }
                left -= 1;
            }

            let mut right = i + 1;
            while right < n {
                if colors[i as usize] != colors[right as usize] {
                    tmp = tmp.max(right - i);
                }
                right += 1;
            }

            ans = ans.max(tmp);
        }

        ans
    }
}