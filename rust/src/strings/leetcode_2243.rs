struct Solution;

impl Solution {
    pub fn digit_sum(s: String, k: i32) -> String {
        let mut v1: Vec<u8> = Vec::with_capacity(s.len());
        let mut v2: Vec<u8> = Vec::with_capacity(s.len());
        let k = k as usize;
        v1.extend(s.as_bytes().iter().map(|&c| c));

        while v1.len() > k {
            v2.clear();

            let len = if v1.len() % k == 0 { v1.len() / k } else { v1.len() / k + 1 };
            for i in 0..len {
                let end = std::cmp::min(v1.len(), (i + 1) * k);
                let slice = &v1[i * k..end];
                let sum: i32 = slice.iter().map(|&c| (c - b'0') as i32).sum();
                let sum = sum.to_string();
                v2.extend_from_slice(sum.as_bytes());
            }

            std::mem::swap(&mut v1, &mut v2);
        }

        unsafe { String::from_utf8_unchecked(v1) }
    }
}