struct Solution;

impl Solution {
    pub fn get_valid_t9_words(num: String, words: Vec<String>) -> Vec<String> {
        let mut ans = Vec::new();
        
        const TABLE: [[u8; 4]; 10] = [
            [b' ', b' ', b' ', b' '],   // 0
            [b'!', b'@', b'#', b' '],   // 1
            [b'a', b'b', b'c', b' '],   // 2
            [b'd', b'e', b'f', b' '],   // 3
            [b'g', b'h', b'i', b' '],   // 4
            [b'j', b'k', b'l', b' '],   // 5
            [b'm', b'n', b'o', b' '],   // 6
            [b'p', b'q', b'r', b's'],   // 7
            [b't', b'u', b'v', b' '],   // 8
            [b'w', b'x', b'y', b'z'],   // 9
        ];

        fn check(num: &str, w: &str) -> bool {
            for (&n, c) in num.as_bytes().iter().zip(w.as_bytes().iter()) {
                let idx = (n - b'0') as usize;
                if !TABLE[idx].contains(c) { return false; }
            }
            true
        }

        for w in words.iter() {
            if w.len() != num.len() { continue; }
            if check(&num, w.as_str()) {
                ans.push(w.clone());
            }
        }

        ans
    }
}