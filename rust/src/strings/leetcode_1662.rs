struct Solution;

impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let len1: usize = word1.iter().map(|w| w.len()).sum();
        let len2: usize = word2.iter().map(|w| w.len()).sum();
        if len1 != len2 { return false; }

        let s1 = word1.concat();
        let s2 = word2.concat();
        s1 == s2
    }
}