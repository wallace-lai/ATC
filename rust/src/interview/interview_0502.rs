struct Solution;

impl Solution {
    pub fn print_bin(mut num: f64) -> String {
        const A: [f64; 7] = [
            0.5,
            0.25,
            0.125,
            0.0625,
            0.03125,
            0.015625,
            0.0078125,
        ];

        let mut ans = String::with_capacity(16);
        ans.push_str("0.");
        for i in A {
            if num >= i {
                num -= i;
                ans.push('1');
            } else if num > 0.0 {
                ans.push('0');
            } else {
                break;
            }
        }

        if num != 0.0 { return "ERROR".to_string(); }
        ans
    }
}