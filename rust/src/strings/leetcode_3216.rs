struct Solution;

impl Solution {
    pub fn get_smallest_string(s: String) -> String {
        let mut v = s.as_bytes().to_vec();
        for i in 0..(v.len() - 1) {
            let flag1 = (v[i] - b'0') & 1;
            let flag2 = (v[i + 1] - b'0') & 1;
            if flag1 == flag2 && v[i] > v[i + 1] {
                let tmp = v[i];
                v[i] = v[i + 1];
                v[i + 1] = tmp;
                break;
            }
        }

        unsafe { String::from_utf8_unchecked(v) }
    }
}