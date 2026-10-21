struct Solution;

impl Solution {
    // 法一：栈，性能差
    // pub fn compress_string(s: String) -> String {
    //     let mut stk: Vec<(u8, u32)> = Vec::with_capacity(s.len());

    //     for &c in s.as_bytes() {
    //         let n = stk.len();
    //         if n == 0 || stk[n - 1].0 != c {
    //             stk.push((c, 1));
    //             continue;
    //         }
    //         stk[n - 1].1 += 1;
    //     }

    //     let mut ans = String::new();
    //     for i in 0..stk.len() {
    //         let (c, count) = stk[i];
    //         ans.push(c as char);
    //         ans.push_str(count.to_string().as_str());
    //     }

    //     if ans.len() >= s.len() { s } else { ans }
    // }

    // 法二：双指针
    pub fn compress_string(s: String) -> String {
        if s.len() == 0 { return "".to_string(); }
        let mut ans = String::with_capacity(s.len());

        let mut i = 0;
        let b = s.as_bytes();
        let n = b.len();
        for k in 0..n {
            if b[k] != b[i] {
                let num = (k - i).to_string();
                ans.push(b[i] as char);
                ans.push_str(&num);

                i = k;
            }
        }
        let num = (n - i).to_string();
        ans.push(b[i] as char);
        ans.push_str(&num);
        
        if ans.len() >= s.len() { s } else { ans }
    }
}