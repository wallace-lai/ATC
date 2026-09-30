struct Solution;

impl Solution {
    pub fn count_valid_words(sentence: String) -> i32 {
        let mut ans = 0;
        for w in sentence.as_str()
            .split_ascii_whitespace()
            .filter(|w| !w.is_empty()) {
            let mut digit = 0;  // 数字
            let mut hyphen = 0; // 破折号
            let mut punctuation = 0;    // 标点符号

            for &c in w.as_bytes() {
                if c.is_ascii_digit() {
                    digit += 1;
                } else if c == b'-' {
                    hyphen += 1;
                } else if c == b'!' || c == b'.' || c == b',' {
                    punctuation += 1;
                }
            }

            let b = w.as_bytes();

            if digit > 0 { continue; }      // 1. 单词中包含数字
            if hyphen >= 2 { continue; }    // 2. 包含两个以上连字符
            if hyphen == 1 {
                // 3. 连字符在单词头部或者尾部
                if b[0] == b'-' || b[b.len() - 1] == b'-' { continue; }
                // 4. 连字符两侧不是小写字母
                if let Some(idx) = w.find('-') {
                    // 此时连字符不可能在单词首尾两端
                    // assert!(0 < idx && idx < w.len() - 1);
                    if !b[idx - 1].is_ascii_lowercase() ||
                        !b[idx + 1].is_ascii_lowercase() {
                        continue;
                    }
                }
            }
            if punctuation >= 2 { continue; }   // 5. 包含两个以上标点符号
            if punctuation == 1 {
                // 6. 标点符号不位于单词末尾
                if b[b.len() - 1] != b'!' &&
                    b[b.len() - 1] != b'.' &&
                    b[b.len() - 1] != b',' {
                    continue;
                }
            }

            ans += 1;
        }

        ans
    }
}