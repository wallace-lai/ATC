struct Solution;

impl Solution {
    pub fn remove_digit(number: String, digit: char) -> String {
        let mut last = usize::MAX;
        let b = number.as_bytes();
        let n = b.len();

        for i in 0..n {
            if b[i] as char == digit {
                if i + 1 < n && b[i] < b[i + 1] {
                    let mut ans = String::with_capacity(n);
                    ans.push_str(&number[0..i]);
                    ans.push_str(&number[i + 1..]);
                    return ans;
                }
                last = i;
            }
        }

        let mut ans = String::with_capacity(n);
        ans.push_str(&number[0..last]);
        ans.push_str(&number[last + 1..]);
        ans
    }
}