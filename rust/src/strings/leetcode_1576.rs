struct Solution;

impl Solution {
    pub fn helper(v: &Vec<u8>) -> u8 {
        let mut ans = b'a';
        for i in b'a'..b'z' {
            if v.iter().all(|c| *c != i) {
                ans = i;
                break;
            }
        }

        ans
    }

    pub fn modify_string(mut s: String) -> String {
        unsafe {
            let b = s.as_bytes_mut();
            let n = b.len();
            let mut v = vec![0_u8; 2];

            for i in 0..n {
                if b[i] == b'?' {
                    v.clear();
                    if i >= 1 { v.push(b[i - 1]); }
                    if i + 1 < n && b[i + 1] != b'?' { v.push(b[i + 1]); }
                    b[i] = Self::helper(&v);
                }
            }
        }

        s
    }
}