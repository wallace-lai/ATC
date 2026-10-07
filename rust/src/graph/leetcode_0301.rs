struct Solution;

use std::collections::HashSet;

impl Solution {
    // 法一：双集合实现BFS
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        fn check(s: &String) -> bool {
            let mut left = 0;
            for c in s.chars() {
                if c == '(' {
                    left += 1;
                } else if c == ')' {
                    if left == 0 {
                        return false;
                    }
                    left -= 1;
                }
            }

            left == 0
        }

        let mut ans: Vec<String> = Vec::new();
        let mut curr: HashSet<String> = HashSet::new();
        let mut next: HashSet<String> = HashSet::new();

        curr.insert(s);
        loop {
            for t in curr.iter() {
                if check(t) {
                    ans.push(t.clone());
                }
            }
            if !ans.is_empty() { return ans; }

            next.clear();
            for t in curr.iter() {
                // 枚举删除t[i]
                for (i, &c) in t.as_bytes().iter().enumerate() {
                    let mut tmp = String::with_capacity(t.len());
                    if c == b'(' || c == b')' {
                        tmp.push_str(&t[0..i]);
                        tmp.push_str(&t[i + 1..]);
                        next.insert(tmp);
                    }
                }
            }

            std::mem::swap(&mut next, &mut curr);
        }
    }
}