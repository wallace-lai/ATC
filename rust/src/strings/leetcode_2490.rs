struct Solution;

impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let mut first = "";
        let mut prev = "";
        for (i, s) in sentence.as_str()
            .split_ascii_whitespace()
            .filter(|s| !s.is_empty())
            .enumerate() {
            if i == 0 {
                first = s;
                prev = s;
                continue;
            }

            if prev.as_bytes()[prev.len() - 1] != s.as_bytes()[0] {
                return false;
            }

            prev = s;
        }

        if prev.as_bytes()[prev.len() - 1] != first.as_bytes()[0] {
            return false;
        }
        // println!("first is {:?}", first);
        // println!("prev is {:?}", prev);

        true
    }
}