struct Solution;

impl Solution {
    // O(n^2)
    // pub fn reverse_parentheses(s: String) -> String {
    //     let n = s.len();
    //     let b = s.as_bytes();
    //     let mut v = Vec::with_capacity(n);
    //     let mut stk = Vec::with_capacity(n);

    //     for i in 0..n {
    //         if b[i] != b')' {
    //             v.push(b[i]);
    //             continue;
    //         }

    //         loop {
    //             let &top = v.last().unwrap();
    //             if top == b'(' {
    //                 v.pop();
    //                 break;
    //             }
    //             stk.push(top);
    //             v.pop();
    //         }

    //         v.extend_from_slice(&stk);
    //         stk.clear();
    //     }

    //     unsafe { String::from_utf8_unchecked(v) }
    // }

    pub fn reverse_parentheses(s: String) -> String {
        let n = s.len();
        let b = s.as_bytes();
        let mut pair = vec![0; n];
        let mut stk = Vec::with_capacity(n);

        for i in 0..n {
            if b[i] == b'(' {
                stk.push(i);
            } else if b[i] == b')' {
                let j = stk.pop().unwrap();
                pair[i] = j;
                pair[j] = i;
            }
        }

        let n = n as i32;
        let mut ans = String::new();
        let mut idx = 0;
        let mut step = 1;

        while idx < n {
            if b[idx as usize] == b'(' ||
                b[idx as usize] == b')' {
                idx = pair[idx as usize] as i32;
                step = -step;
            } else {
                ans.push(b[idx as usize] as char);
            }
            idx += step;
        }

        ans
    }
}