struct Solution;

impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let mut ans = 0;
        let mut v = Vec::with_capacity(s.len());

        let mut count = 1;
        let mut last = s.as_bytes()[0];
        for i in 1..s.len() {
            if s.as_bytes()[i] == last {
                count += 1;
            } else {
                last = s.as_bytes()[i];
                v.push(count);
                count = 1;
            }
        }
        v.push(count);
        // println!("v is {:?}", v);

        for i in 1..v.len() {
            ans += v[i - 1].min(v[i]);
        }

        ans
    }
}