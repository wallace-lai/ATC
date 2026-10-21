struct Solution;

impl Solution {
    pub fn is_fliped_string(a: String, b: String) -> bool {
        if a.len() != b.len() { return false; }
        let mut c = String::with_capacity(a.len() * 2);
        c.push_str(&a);
        c.push_str(&a);
        c.contains(&b)
    }
}