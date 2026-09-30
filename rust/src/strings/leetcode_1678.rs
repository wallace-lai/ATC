struct Solution;

impl Solution {
    pub fn interpret(command: String) -> String {
        let mut v: Vec<u8> = Vec::new();
        let b = command.as_bytes();
        let n = b.len();

        let mut i = 0;
        while i < n {
            if i + 4 <= n && &b[i..(i + 4)] == "(al)".as_bytes() {
                v.extend([b'a', b'l'].iter());
                i += 4;
                continue;
            }

            if i + 2 <= n && &b[i..(i + 2)] == "()".as_bytes() {
                v.push(b'o');
                i += 2;
                continue;
            }

            if b[i] == b'G' {
                v.push(b'G');
                i += 1;
            }
        }

        unsafe { String::from_utf8_unchecked(v) }
    }
}