struct Solution;

impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        let mut v = Vec::with_capacity(s.len());
        let mut has_vline = false;

        let b = s.as_bytes();
        for i in 0..b.len() {
            if b[i] == b'|' {
                if has_vline {
                    // 出栈直到上一个竖线被抛出
                    while let Some(top) = v.pop() {
                        if top == b'|' { break; }
                    }
                    has_vline = false;
                } else {
                    v.push(b'|');
                    has_vline = true;
                }
                continue;
            }

            v.push(b[i]);
        }

        v.iter().filter(|&c| *c == b'*').count() as i32
    }
}