struct Solution;

impl Solution {
    pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let n = groups.len();
        let mut v1 = Vec::with_capacity(n);
        let mut v2 = Vec::with_capacity(n);

        for i in 0..n {
            v2.clear();
            v2.push(i);
            for j in (i + 1)..n {
                let &last = v2.last().unwrap();
                if groups[j] != groups[last] {
                    v2.push(j);
                }
            }

            if v2.len() > v1.len() {
                std::mem::swap(&mut v1, &mut v2);
            }
        }

        let mut ans = Vec::with_capacity(n);
        for idx in v1 {
            ans.push(words[idx].clone());
        }

        ans
    }
}