struct NumArray {
    n: usize,
    b: Vec<i64>,
}

impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        let len = nums.len();
        let mut ans = NumArray {
            n: len,
            b: vec![0; len + 1],
        };

        for (i, &x) in nums.iter().enumerate() {
            ans.add(i + 1, x);
        }
        ans
    }

    // 1-indexed
    fn add(&mut self, mut i: usize, delta: i32) {
        while i <= self.n {
            self.b[i] += delta as i64;
            i += i & i.wrapping_neg();
        }
    }

    // 1-indexed
    fn query(&self, mut i: usize) -> i32 {
        let mut ans = 0;
        while i > 0 {
            ans += self.b[i];
            i -= i & i.wrapping_neg();
        }
        ans as i32
    }
    
    // 0-indexed
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let (l, r) = (left as usize, right as usize + 1);
        self.query(r) - self.query(l)
    }
}