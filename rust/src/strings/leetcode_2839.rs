struct Solution;

impl Solution {
    pub fn swap(v: &mut Vec<u8>, i: usize, j: usize) {
        let tmp = v[i];
        v[i] = v[j];
        v[j] = tmp;
    }

    pub fn can_be_equal(s1: String, s2: String) -> bool {
        let mut v1 = Vec::with_capacity(s1.len());
        let mut v2 = Vec::with_capacity(s2.len());
        v1.extend_from_slice(s1.as_bytes());
        v2.extend_from_slice(s2.as_bytes());
        if v1 == v2 { return true; }

        Self::swap(&mut v1, 0, 2);
        if v1 == v2 { return true; }
        Self::swap(&mut v1, 0, 2);

        Self::swap(&mut v1, 1, 3);
        if v1 == v2 { return true; }
        Self::swap(&mut v1, 1, 3);

        Self::swap(&mut v1, 0, 2);
        Self::swap(&mut v1, 1, 3);
        if v1 == v2 { return true; }

        false
    }
}