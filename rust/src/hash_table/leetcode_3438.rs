struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn find_valid_pair(s: String) -> String {
        let mut count = [0; 16];
        for &c in s.as_bytes() {
            let idx = (c - b'0') as usize;
            count[idx] += 1;
        }

        // println!("len is {}", s.as_bytes().len());

        let mut ans = "".to_string();
        let str = s.as_bytes();
        for i in 1..str.len() {
            if str[i - 1] != str[i] {
                let idx1 = (str[i - 1] - b'0') as usize;
                let idx2 = (str[i] - b'0') as usize;
                if count[idx1] as usize == idx1 && count[idx2] as usize == idx2 {
                    ans.push(str[i - 1] as char);
                    ans.push(str[i] as char);
                    break;
                }
            }
        }

        ans
    }
}