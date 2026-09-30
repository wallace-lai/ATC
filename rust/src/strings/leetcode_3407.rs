struct Solution;

impl Solution {
    // 1ms
    // pub fn has_match(s: String, p: String) -> bool {
    //     let pb = p.as_bytes();
    //     if pb[0] == b'*' {
    //         let ps = &p.as_str()[1..];
    //         if s.contains(ps) { return true; }
    //     } else if pb[pb.len() - 1] == b'*' {
    //         let n = p.as_str().len();
    //         let ps = &p.as_str()[0..(n - 1)];
    //         if s.contains(ps) { return true; }
    //     } else {
    //         let v: Vec<&str> = p.split('*').collect();
    //         // println!("v is {:?}", v);
    //         if let Some((idx1, idx2)) = s.find(v[0]).zip(s.rfind(v[1])) {
    //             if idx2 >= idx1 + v[0].len() {
    //                 return true;
    //             }
    //         }
    //     }

    //     false
    // }

    // 0ms
    pub fn has_match(s: String, p: String) -> bool {
        let (pre, suf) = p.split_once('*').unwrap();
        match (s.find(pre), s.rfind(suf)) {
            (Some(idx1), Some(idx2)) => idx2 >= idx1 + pre.len(),
            _ => false,
        }
    }
}