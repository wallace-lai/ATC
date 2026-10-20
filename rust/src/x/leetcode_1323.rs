struct Solution;

impl Solution {
    pub fn maximum69_number (num: i32) -> i32 {

        fn helper(mut n: i32, v: &mut Vec<i32>) {
            while n > 0 {
                v.push(n % 10);
                n /= 10;
            }
        }

        let mut v = Vec::new();
        helper(num, &mut v);
        for i in (0..v.len()).rev() {
            if v[i] == 6 {
                v[i] = 9;
                break;
            }
        }

        let mut ans = 0;
        for i in (0..v.len()).rev() {
            ans = ans * 10 + v[i];
        }
        ans
    }
}