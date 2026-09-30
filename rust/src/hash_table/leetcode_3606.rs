struct Solution;

impl Solution {
    pub fn is_valid_ascii(s: &str) -> bool {
        s.bytes().all(|b| b == b'_' || b.is_ascii_alphanumeric())
    }

    // 0ms，击败100%
    pub fn validate_coupons(code: Vec<String>, business_line: Vec<String>, is_active: Vec<bool>) -> Vec<String> {
        // e, g, p, r
        let mut lines: Vec<Vec<String>> = vec![Vec::new(); 4];

        for i in 0..code.len() {
            if !is_active[i] { continue; }
            if code[i] == "".to_string() || !Self::is_valid_ascii(&code[i]) {
                continue;
            }

            match business_line[i].as_str() {
                "electronics" => {lines[0].push(code[i].clone());},
                "grocery" => {lines[1].push(code[i].clone());},
                "pharmacy" => {lines[2].push(code[i].clone());},
                "restaurant" => {lines[3].push(code[i].clone());},
                _ => ()
            }
        }

        for line in &mut lines {
            line.sort_unstable();
        }
        
        lines.into_iter().flatten().collect()
    }
}