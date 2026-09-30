struct Solution;

impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        if n & 1 == 1 {
            "a".repeat(n as usize)
        } else {
            let mut ans = "a".repeat(n as usize - 1);
            ans.push(b'b' as char);
            ans
        }
    }
}