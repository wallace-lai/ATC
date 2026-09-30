struct Solution;

use std::collections::HashMap;

impl Solution {
    const TABLE: [(&str, u8); 26] = [
        ("1", b'a'), ("2", b'b'), ("3", b'c'), ("4", b'd'),
        ("5", b'e'), ("6", b'f'), ("7", b'g'), ("8", b'h'),
        ("9", b'i'), 
        ("10#", b'j'), ("11#", b'k'), ("12#", b'l'),
        ("13#", b'm'), ("14#", b'n'), ("15#", b'o'),
        ("16#", b'p'), ("17#", b'q'), ("18#", b'r'),
        ("19#", b's'), ("20#", b't'), ("21#", b'u'),
        ("22#", b'v'), ("23#", b'w'), ("24#", b'x'),
        ("25#", b'y'), ("26#", b'z')
    ];

    pub fn freq_alphabets(s: String) -> String {
        let m: HashMap<&str, u8> = Self::TABLE.iter()
            .map(|(str, c)| (*str, *c))
            .collect();

        let b = s.as_str();
        let mut i = 0;
        let mut ans: Vec<u8> = Vec::new();
        while i < b.len() {
            if i + 3 <= b.len() {
                if let Some(val) = m.get(&b[i..(i + 3)]) {
                    ans.push(*val);
                    i += 3;
                    continue;
                }
            }

            if let Some(val) = m.get(&b[i..(i + 1)]) {
                ans.push(*val);
            }

            i += 1;
        }

        unsafe { String::from_utf8_unchecked(ans) }
    }
}