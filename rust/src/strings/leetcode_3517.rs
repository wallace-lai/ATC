struct Solution;

impl Solution {
    pub fn num_to_char(n: u8) -> char {
        assert!(n < 26, "n must be between 0 and 25.");
        (b'a' + n) as char
    }

    pub fn char_to_num(c: char) -> Option<u8> {
        if c.is_ascii_lowercase() {
            Some(c as u8 - b'a')
        } else {
            None
        }
    }

    // 15ms
    // pub fn smallest_palindrome(s: String) -> String {
    //     let mut count = [0_u32; 26];
    //     for &c in s.as_bytes() {
    //         let idx = (c - b'a') as usize;
    //         count[idx] += 1;
    //     }

    //     let mut left = 0_usize;
    //     let mut right = s.len() - 1;
    //     let mut vstr = vec![0_u8; s.len()];
    //     for i in 0..count.len() {
    //         if count[i] == 0 {
    //             continue;
    //         }

    //         let c = i as u8 + b'a';
    //         if count[i] % 2 == 1 {
    //             // 奇数个的字母只能有一个且此时字符串长度为奇数
    //             // 将该奇数个的字母填充到回文串的中间
    //             assert!(s.len() % 2 == 1);
    //             count[i] -= 1;
    //             vstr[s.len() / 2] = c;
    //         }

    //         while count[i] > 0 {
    //             vstr[left] = c;
    //             vstr[right] = c;
    //             left += 1;
    //             right -= 1;
    //             count[i] -= 2;
    //         }            
    //     }

    //     vstr.iter().map(|&b| b as char).collect()
    // }

    pub fn smallest_palindrome(s: String) -> String {
        let mut count = [0; 26];
        for &c in s.as_bytes() {
            count[(c - b'a') as usize] += 1;
        }

        let mut mid: u8 = 0;
        let mut append_mid = false;

        let mut left = Vec::<u8>::with_capacity(s.len());
        for i in 0..count.len() {
            if count[i] > 0 {
                left.extend(vec![i as u8 + b'a'; count[i] / 2]);

                if count[i] % 2 == 1 {
                    mid = i as u8 + b'a';
                    append_mid = true;
                }
            } 
        }
        let mut right = left.clone();
        right.reverse();

        if append_mid {
            left.push(mid);
        }
        left.extend(right);

        String::from_utf8(left).unwrap()
    }
}
