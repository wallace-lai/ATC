struct Solution;

impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>) -> i32 {
        let mut ans = 0;
        let n = colors.len();   // n >= 3
        
        // i == 0
        if colors[0] != colors[n - 1] && colors[0] != colors[1] {
            ans += 1;
        }
        // i == n - 1
        if colors[n - 1] != colors[n - 2] && colors[n - 1] != colors[0] {
            ans += 1;
        }

        for i in 1..(n - 1) {
            if colors[i] != colors[i - 1] && colors[i] != colors[i + 1] {
                ans += 1
            }
        }

        ans
    }
}