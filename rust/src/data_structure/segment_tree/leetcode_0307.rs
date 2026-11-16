struct NumArray {
    n: usize,
    b: Vec<i64>,
    v: Vec<i32>,
}


impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        let len = nums.len();
        let mut na = NumArray {
            n: len,
            b: vec![0; len + 1],
            v: vec![0; len],
        };

        for (i, &x) in nums.iter().enumerate() {
            na.add(i + 1, x);
        }
        na
    }

    // 1-indexed
    fn add(&mut self, mut i: usize, delta: i32) {
        self.v[i - 1] += delta;

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
    fn update(&mut self, index: i32, val: i32) {
        let i = index as usize;
        let delta = val - self.v[i];
        self.add(i + 1, delta);
    }
    
    // 0-indexed
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let (l, r) = (left as usize, right as usize + 1);
        self.query(r) - self.query(l)
    }
}
