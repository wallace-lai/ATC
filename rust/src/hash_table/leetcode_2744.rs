struct Solution;

impl Solution {
    pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
        let len = words.len();
        let mut ans = 0;
        for i in 0..len {
            for j in (i + 1)..len {
                let si = words[i].as_bytes();
                let sj = words[j].as_bytes();
                if si[0] == sj[1] && si[1] == sj[0] {
                    ans += 1;
                };
            }
        }

        ans
    }
}