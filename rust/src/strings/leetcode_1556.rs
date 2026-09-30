struct Solution;

impl Solution {
    // pub fn thousand_separator(n: i32) -> String {
    //     let s = n.to_string();
    //     let b = s.as_bytes();
    //     let mut ans = String::with_capacity(s.len() + 3);

    //     match s.len() {
    //         1 | 2 | 3 => {
    //             ans.extend(s.chars());
    //         },
    //         4 => {
    //             ans.push(b[0] as char);
    //             ans.push(b'.' as char);
    //             ans.push_str(&s[1..]);
    //         },
    //         5 => {
    //             ans.push_str(&s[0..2]);
    //             ans.push(b'.' as char);
    //             ans.push_str(&s[2..]);
    //         },
    //         6 => {
    //             ans.push_str(&s[0..3]);
    //             ans.push(b'.' as char);
    //             ans.push_str(&s[3..]);
    //         },
    //         7 => {
    //             ans.push(b[0] as char);
    //             ans.push(b'.' as char);
    //             ans.push_str(&s[1..4]);
    //             ans.push(b'.' as char);
    //             ans.push_str(&s[4..]);
    //         },
    //         8 => {
    //             ans.push_str(&s[0..2]);
    //             ans.push(b'.' as char);
    //             ans.push_str(&s[2..5]);
    //             ans.push(b'.' as char);
    //             ans.push_str(&s[5..]);
    //         },
    //         9 => {
    //             ans.push_str(&s[0..3]);
    //             ans.push(b'.' as char);
    //             ans.push_str(&s[3..6]);
    //             ans.push(b'.' as char);
    //             ans.push_str(&s[6..]);
    //         },
    //         10 => {
    //             ans.push(b[0] as char);
    //             ans.push(b'.' as char);
    //             ans.push_str(&s[1..4]);
    //             ans.push(b'.' as char);
    //             ans.push_str(&s[4..7]);
    //             ans.push(b'.' as char);
    //             ans.push_str(&s[7..]);
    //         },
    //         _ => {}
    //     }

    //     ans
    // }

    pub fn thousand_separator(n: i32) -> String {
        let b = n.to_string().into_bytes();
        let n = b.len();
        let mut ans = String::with_capacity(n + 3);

        for i in 0..n {
            if i > 0 && (n - i) % 3 == 0 {
                ans.push('.');
            }
            ans.push(b[i] as char);
        }

        ans
    }
}