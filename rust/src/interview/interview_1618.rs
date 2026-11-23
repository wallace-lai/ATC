struct Solution;

impl Solution {
    // 法一：枚举
    pub fn pattern_matching(pattern: String, value: String) -> bool {
        let p: Vec<char> = pattern.chars().collect();
        let v: Vec<char> = value.chars().collect();

        // 边界：pattern 为空
        if p.is_empty() { return v.is_empty(); }

        let count_a = p.iter().filter(|&&c| c == 'a').count();
        let count_b = p.len() - count_a;

        // 让 c1 表示出现次数较多的字符
        let (c1, count1, c2, count2) = if count_a > count_b {
            ('a', count_a, 'b', count_b)
        } else {
            ('b', count_b, 'a', count_a)
        };

        // 情况一：只有一种字符，直接等分验证
        if count2 == 0 {
            if v.len() % count1 != 0 {
                return false;
            }
            let len = v.len() / count1;
            let first = &v[0..len];
            for i in 1..count1 {
                let s = i * len;
                if &v[s..s + len] != first {
                    return false;
                }
            }
            return true;
        }

        // 情况二：两种字符都有，枚举 len1
        // len1 的范围：0 到 v.len() / count1
        let max_len1 = v.len() / count1;

        for len1 in 0..=max_len1 {
            let remain = v.len() - count1 * len1;
            if remain % count2 != 0 {
                continue;
            }
            let len2 = remain / count2;

            // 遍历 pattern 逐个截取子串并验证
            let mut start1: Option<usize> = None; // c1 代表的子串起始位置
            let mut start2: Option<usize> = None; // c2 代表的子串起始位置
            let mut pos = 0;
            let mut valid = true;

            for &ch in &p {
                if ch == c1 {
                    let sub = &v[pos..pos + len1];
                    match start1 {
                        Some(s) => {
                            if &v[s..s + len1] != sub {
                                valid = false;
                                break;
                            }
                        }
                        None => start1 = Some(pos),
                    }
                    pos += len1;
                } else {
                    let sub = &v[pos..pos + len2];
                    match start2 {
                        Some(s) => {
                            if &v[s..s + len2] != sub {
                                valid = false;
                                break;
                            }
                        }
                        None => start2 = Some(pos),
                    }
                    pos += len2;
                }
            }

            if !valid {
                continue;
            }

            // 两者都出现过时，必须保证 c1 与 c2 代表的子串不同
            if let (Some(s1), Some(s2)) = (start1, start2) {
                if len1 == len2 && v[s1..s1 + len1] == v[s2..s2 + len2] {
                    continue;
                }
            }

            return true;
        }

        false        
    }
}