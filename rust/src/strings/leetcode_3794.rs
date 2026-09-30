struct Solution;

impl Solution {
    pub fn reverse_prefix(s: String, k: i32) -> String {
        let mut v: Vec<char> = s.chars().collect();
        let mut left = 0;
        let mut right = k - 1;

        while left < right {
            let tmp = v[left as usize];
            v[left as usize] = v[right as usize];
            v[right as usize] = tmp;

            left += 1;
            right -= 1;
        }

        String::from_iter(v.into_iter())
    }
}