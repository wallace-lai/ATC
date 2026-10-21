struct Solution;

impl Solution {
    // 法一：枚举
    // pub fn one_edit_away(s1: String, s2: String) -> bool {
    //     let n1 = s1.len();
    //     let n2 = s2.len();
    //     if n1.abs_diff(n2) > 1 { return false; }

    //     // a == b1 + b2
    //     fn slice_compare(a: &[u8], b1: &[u8], b2: &[u8]) -> bool {
    //         &a[0..b1.len()] == b1 && &a[b1.len()..] == b2
    //     }

    //     if n1 == n2 {
    //         let count = s1.as_bytes().iter()
    //             .zip(s2.as_bytes().iter())
    //             .filter(|&(c1, c2)| c1 != c2)
    //             .count();
    //         return count < 2;
    //     }

    //     // a为较短的字符串
    //     let (a, b) = if n1 < n2 {
    //         (s1.as_bytes(), s2.as_bytes())
    //     } else {
    //         (s2.as_bytes(), s1.as_bytes())
    //     };

    //     if a == &b[1..] || a == &b[0..(b.len() - 1)] {
    //         return true;
    //     }

    //     for i in 1..(b.len() - 1) {
    //         if slice_compare(a, &b[0..i], &b[i + 1..]) {
    //             return true;
    //         }
    //     }

    //     false
    // }

    // 法二：双指针
    pub fn one_edit_away(a: String, b: String) -> bool {
        let la = a.len();
        let lb = b.len();
        // 确保a是较长的字符串
        if la < lb { return Self::one_edit_away(b, a); }
        if la - lb > 1 { return false; }

        if la == lb {
            let count = a.as_bytes()
                .iter()
                .zip(b.as_bytes().iter())
                .filter(|&(c1, c2)| c1 != c2)
                .count();
            return count < 2;
        }

        let mut i = 0;
        let mut o = 0;
        while i < lb {
            if b.as_bytes()[i] != a.as_bytes()[i + o] {
                o += 1;
                if o > 1 { return false; }
            } else {
                i += 1;
            }
        }

        true
    }
}