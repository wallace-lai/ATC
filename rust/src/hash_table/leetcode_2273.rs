struct Solution;

impl Solution {
    pub fn build_count(count: &mut [u8], s: &String) {
        for &c in s.as_bytes() {
            let idx = (c - b'a') as usize;
            count[idx] += 1;
        }
    }

    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let n = words.len();
        let mut ans = Vec::with_capacity(words.len());
        let mut cnti = [0u8; 26];
        let mut cntj = [0u8; 26];

        let mut i = 0;
        let mut j = i + 1;
        while i < n {
            ans.push(words[i].clone());
            cnti.fill(0);
            Self::build_count(&mut cnti, &words[i]);
            
            while j < n {
                cntj.fill(0);
                Self::build_count(&mut cntj, &words[j]);
                if cnti != cntj {
                    break;
                }
                j += 1;
            }

            i = j;
        }

        ans
    }
}