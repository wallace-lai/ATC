struct Solution;

impl Solution {
    pub fn string_matching(mut words: Vec<String>) -> Vec<String> {
        let mut ans = Vec::with_capacity(words.len());
        
        words.sort_unstable_by_key(|w| w.len());
        for i in 0..words.len() {
            for j in (i + 1)..words.len() {
                if words[j].contains(words[i].as_str()) {
                    ans.push(words[i].clone());
                    break;
                }
            }
        }

        ans
    }
}