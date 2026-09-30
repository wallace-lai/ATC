struct Solution;

impl Solution {
    pub fn reverse_by_type(s: String) -> String {
        let mut v = Vec::from_iter(s.chars());
        // println!("v is {:?}", v);

        let n = s.len() as i32;
        let mut left = 0;
        let mut right = n - 1;
        while left < right {
            while left < n && !v[left as usize].is_ascii_lowercase() {
                left += 1;
            }
            while right >= 0 && !v[right as usize].is_ascii_lowercase() {
                right -= 1;
            }
            if left < right {
                let tmp = v[left as usize];
                v[left as usize] = v[right as usize];
                v[right as usize] = tmp;

                left += 1;
                right -= 1;
            }
        }

        left = 0;
        right = n - 1;
        while left < right {
            while left < n && v[left as usize].is_ascii_alphabetic() {
                left += 1;
            }
            while right >= 0 && v[right as usize].is_ascii_alphabetic() {
                right -= 1;
            }
            if left < right {
                let tmp = v[left as usize];
                v[left as usize] = v[right as usize];
                v[right as usize] = tmp;

                left += 1;
                right -= 1;
            }
        }
        

        String::from_iter(v.into_iter())
    }
}