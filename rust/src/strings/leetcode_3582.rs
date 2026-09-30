struct Solution;

impl Solution {
    pub fn generate_tag(caption: String) -> String {
        let mut v = Vec::with_capacity(caption.len() + 1);
        v.push(b'#');

        for s in caption.split_whitespace()
            .filter(|s|!s.is_empty()) {
            // println!("s is {:?}", s);
            let mut tmp = Vec::from(s);
            tmp.make_ascii_lowercase();
            tmp[0] = tmp[0].to_ascii_uppercase();
            v.extend(tmp);
        }
        if v.len() == 1 { return "#".to_string(); }

        v[1] = v[1].to_ascii_lowercase();
        v.truncate(100);
        unsafe { String::from_utf8_unchecked(v) }
    }
}