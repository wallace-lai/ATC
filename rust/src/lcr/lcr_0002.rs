struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let astr = a.as_bytes();
        let bstr = b.as_bytes();
        let cap = a.len().max(b.len()) + 1;
        let mut v: Vec<u8> = Vec::with_capacity(cap);

        let mut i = a.len() as i32 - 1;
        let mut j = b.len() as i32 - 1;
        let mut sum;
        let mut carry = 0;
        while i >= 0 && j >= 0 {
            sum = carry + astr[i as usize] + bstr[j as usize] - b'0' * 2;
            carry = if sum >= 2 { sum / 2 } else { 0 };
            sum %= 2;

            v.push(sum + b'0');
            i -= 1;
            j -= 1;
        }

        while i >= 0 {
            sum = carry + astr[i as usize] - b'0';
            carry = if sum >= 2 { sum / 2 } else { 0 };
            sum %= 2;

            v.push(sum + b'0');
            i -= 1;
        }
        while j >= 0 {
            sum = carry + bstr[j as usize] - b'0';
            carry = if sum >= 2 { sum / 2 } else { 0 };
            sum %= 2;

            v.push(sum + b'0');
            j -= 1;
        }
        if carry > 0 { v.push(carry + b'0'); }

        v.reverse();
        unsafe { String::from_utf8_unchecked(v) }
    }
}