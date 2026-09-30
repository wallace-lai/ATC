struct Solution;

impl Solution {
    // time : O(n + m)
    // space : O(n + m)
    // 0ms，击败100%
    // pub fn backspace_compare(s: String, t: String) -> bool {
    //     let mut sv = Vec::with_capacity(s.len());
    //     let mut tv = Vec::with_capacity(t.len());

    //     for &c in s.as_bytes() {
    //         if c == b'#' {
    //             sv.pop();
    //         } else {
    //             sv.push(c);
    //         }
    //     }
        
    //     for &c in t.as_bytes() {
    //         if c == b'#' {
    //             tv.pop();
    //         } else {
    //             tv.push(c);
    //         }
    //     }

    //     sv == tv
    // }

    // time : O(n + m)
    // space : O(1)
    // 0ms，击败100%
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut i = s.len() as i32 - 1;
        let mut j = t.len() as i32 - 1;
        let mut skip_s = 0;
        let mut skip_t = 0;

        while i >= 0 || j >= 0 {
            while i >= 0 {
                if s.as_bytes()[i as usize] == b'#' {
                    skip_s += 1;
                    i -= 1;
                } else if skip_s > 0 {
                    skip_s -= 1;
                    i -= 1;
                } else {
                    break;
                }
            }

            while j >= 0 {
                if t.as_bytes()[j as usize] == b'#' {
                    skip_t += 1;
                    j -= 1;
                } else if skip_t > 0 {
                    skip_t -= 1;
                    j -= 1;
                } else {
                    break;
                }
            }

            if i >= 0 && j >= 0 {
                if s.as_bytes()[i as usize] != t.as_bytes()[j as usize] {
                    return false;
                }
            } else if i >= 0 || j >= 0 {
                return false;
            }

            i -= 1;
            j -= 1;
        }

        true
    }
}