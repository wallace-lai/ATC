struct Solution;

impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        let mut ans = String::with_capacity(sentence.len() * 2);
        for (i, s) in sentence.as_str().split_whitespace().filter(|s| !s.is_empty()).enumerate() {
            let c = s.chars().nth(0).unwrap().to_ascii_lowercase();
            if c == 'a' || c == 'e' || c == 'i' ||  c == 'o' || c == 'u' {
                ans.push_str(s);
                ans.push_str("ma");
            } else {
                ans.push_str(&s[1..]);
                ans.push(s.chars().nth(0).unwrap());
                ans.push_str("ma");
            }

            ans.push_str(&"a".repeat(i + 1));
            ans.push(' ');
        }

        ans.pop();
        ans
    }
}