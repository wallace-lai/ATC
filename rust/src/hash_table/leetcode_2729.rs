struct Solution;

impl Solution {
    pub fn is_fascinating(n: i32) -> bool {
        let mut count = [0; 10];
        
        let mut num = n;
        while num > 0 {
            let idx = (num % 10) as usize;
            count[idx] += 1;
            num /= 10;
        }

        num = 2 * n;
        while num > 0 {
            let idx = (num % 10) as usize;
            count[idx] += 1;
            num /= 10;
        }

        num = 3 * n;
        while num > 0 {
            let idx = (num % 10) as usize;
            count[idx] += 1;
            num /= 10;
        }

        if count[0] > 0 { return false; }
        for i in 1..10 {
            if count[i] != 1 { return false; }
        }

        true        
    }
}