struct Solution;

impl Solution {
    const ONES: [&str; 20] = [
        "", "One", "Two", "Three", "Four", "Five",
        "Six", "Seven", "Eight", "Nine", "Ten",
        "Eleven", "Twelve", "Thirteen", "Fourteen",
        "Fifteen", "Sixteen", "Seventeen", "Eighteen",
        "Nineteen"
    ];

    const TENS: [&str; 10] = [
        "", "", "Twenty", "Thirty", "Forty",
        "Fifty", "Sixty", "Seventy", "Eighty",
        "Ninety"
    ];

    const THOUSANDS: [&str; 4] = [
        "", "Thousand", "Million", "Billion"
    ];

    fn below_1000(n: usize) -> String {
        assert!(n <= 999);
        match n {
            0 => String::new(),
            1..=19 => Self::ONES[n].to_string(),
            20..=99 => {
                let t = Self::TENS[n / 10];
                let o = Self::ONES[n % 10];
                if o.is_empty() {
                    t.to_string()
                } else {
                    format!("{t} {o}")
                }
            },
            _ => {
                // 100..=999
                let head = format!("{} Hundred", Self::ONES[n / 100]);
                let rest = Self::below_1000(n % 100);
                if rest.is_empty() {
                    head
                } else {
                    format!("{head} {rest}")
                }
            }
        }
    }

    pub fn number_to_words(num: i32) -> String {
        if num == 0 { return "Zero".to_string(); }

        let mut n = num as usize;
        let mut parts = Vec::new();
        let mut k = 0;

        while n > 0 {
            let chunk = n % 1000;
            if chunk != 0 {
                let mut s = Self::below_1000(chunk);
                if !Self::THOUSANDS[k].is_empty() {
                    s.push(' ');
                    s.push_str(Self::THOUSANDS[k]);
                }
                parts.push(s);
            }

            n /= 1000;
            k += 1;
        }

        parts.reverse();
        parts.join(" ")
    }
}