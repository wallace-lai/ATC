struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut unique_emails = HashSet::with_capacity(emails.len());
        for email in emails.into_iter() {
            // 1. 使用 split_once 一次分割，直接得到 (&str, &str)
            if let Some((local, domain)) = email.split_once('@') {
                // 2. 查找'+'，并截取之前的部分
                let local = match local.find('+') {
                    Some(plus_idx) => &local[..plus_idx],
                    None => local,
                };

                // 3. 移除所有的'.'号，得到规范后的本地名
                let normalized_local: String = local
                    .chars()
                    .filter(|&c| c != '.')
                    .collect();

                unique_emails.insert(format!("{}@{}", normalized_local, domain));
            }
        }

        unique_emails.len() as i32
    }
}