struct Solution;

impl Solution {
    pub fn get_lucky(s: String, mut k: i32) -> i32 {
        let mut num = 0;
        let b = s.as_bytes();
        let n = b.len();

        for i in 0..n {
            num += match b[i] {
                b'a' | b'j' => { 1 },
                b'b' | b'k' | b't' => { 2 },
                b'c' | b'l' | b'u' => { 3 },
                b'd' | b'm' | b'v' => { 4 },
                b'e' | b'n' | b'w' => { 5 },
                b'f' | b'o' | b'x' => { 6 },
                b'g' | b'p' | b'y' => { 7 },
                b'h' | b'q' | b'z' => { 8 },
                b'i' | b'r' => { 9 },
                b's' => { 10 },
                _ => { 0 }
            }
        }
        k -= 1;

        while k > 0 {
            num = {
                let mut ans = 0;
                let mut n = num;
                while n > 0 {
                    ans += n % 10;
                    n /= 10;
                }
                ans
            };

            k -= 1;
        }

        num
    }
}